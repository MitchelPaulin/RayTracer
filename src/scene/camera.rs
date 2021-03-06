use std::{sync::Arc, thread};

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};

use crate::{
    draw::canvas::{stitch_canvases, Canvas},
    math::{matrix::Matrix, ray::Ray, tuples::Tuple},
};

use super::world::World;
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new_with_transform(
        hsize: usize,
        vsize: usize,
        field_of_view: f64,
        transform: Matrix,
    ) -> Camera {
        let mut c = Camera::new(hsize, vsize, field_of_view);
        c.transform = transform;
        c
    }

    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        // the length of half of the fov
        let half_view = (field_of_view / 2.0).tan();
        let aspect_ratio = hsize as f64 / vsize as f64;
        let half_width;
        let half_height;

        if aspect_ratio >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect_ratio;
        } else {
            half_height = half_view;
            half_width = half_view * aspect_ratio;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(4),
            pixel_size,
            half_width,
            half_height,
        }
    }
    /*
        For any pixel in the scene calculate a ray which
        would intersect that pixel
    */
    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the center of the pixel we are targeting
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        // the coordinates of the pixel in world space
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inv = self.transform.inverse();

        let pixel = &inv * &Tuple::point(world_x, world_y, -1.0);
        let origin = &inv * &Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }
}

pub fn render(camera: Camera, world: World, thread_count: usize) -> Canvas {
    assert!(thread_count >= 1);

    let vsize_per_thread = camera.vsize / thread_count;
    let last_thread_offset = camera.vsize % thread_count;
    let mut children = vec![];

    let multi_progress_bar = MultiProgress::new();
    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");
    multi_progress_bar.set_draw_target(ProgressDrawTarget::stdout());
    let c = Arc::new(camera);
    let w = Arc::new(world);

    for thread_num in 0..thread_count {
        let cc = c.clone();
        let wc = w.clone();
        let progress_bar = if thread_num < thread_count - 1 {
            multi_progress_bar.add(ProgressBar::new(vsize_per_thread as u64))
        } else {
            multi_progress_bar.add(ProgressBar::new(
                (vsize_per_thread + last_thread_offset) as u64,
            ))
        };
        progress_bar.set_style(progress_style.clone());

        if thread_num == 0 {
            progress_bar.println(format!("Rendering image on {} thread(s)", thread_count));
        }

        if thread_num < thread_count - 1 {
            children.push(thread::spawn(move || {
                (
                    render_thread(
                        cc,
                        wc,
                        vsize_per_thread * thread_num,
                        vsize_per_thread * (thread_num + 1),
                        thread_num,
                        progress_bar,
                    ),
                    thread_num,
                )
            }));
        } else {
            // we need to add the offset to the last thread in case the number of threads does not evenly divide the height
            children.push(thread::spawn(move || {
                (
                    render_thread(
                        cc,
                        wc,
                        vsize_per_thread * thread_num,
                        vsize_per_thread * (thread_num + 1) + last_thread_offset,
                        thread_num,
                        progress_bar,
                    ),
                    thread_num,
                )
            }));
        }
    }

    // must join progress bar handle before threads or nothing will render
    multi_progress_bar.join().unwrap();

    let mut result = vec![];
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let handle = child.join().unwrap();
        result.push(handle);
    }

    // stitch the resulting images together
    result.sort_by(|c1, c2| c1.1.cmp(&c2.1));
    let mut canvases = vec![];
    for c in result {
        canvases.push(c.0);
    }
    stitch_canvases(canvases)
}

fn render_thread(
    camera: Arc<Camera>,
    world: Arc<World>,
    thread_y_start: usize,
    thread_y_end: usize,
    thread_number: usize,
    progress_bar: ProgressBar,
) -> Canvas {
    let mut image = Canvas::new(camera.hsize, thread_y_end - thread_y_start);
    for y in thread_y_start..thread_y_end {
        progress_bar.inc(1);
        for x in 0..camera.hsize {
            let ray = camera.ray_for_pixel(x, y);
            let color = world.color_at(&ray, 5);
            image.write_pixel(x, y - thread_y_start, color);
        }
    }
    progress_bar.finish_with_message(format!("Thread {} done", thread_number));
    image
}

/*
    Move the eye to a new point in the scene
*/
pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
    assert!(from.is_point());
    assert!(to.is_point());
    assert!(up.is_vector());

    let forward = (to - from).normalize();
    let left = forward.cross(&up.normalize());
    let true_up = left.cross(&forward);

    let orientation = Matrix {
        size: 4,
        matrix: vec![
            vec![left.x, left.y, left.z, 0.0],
            vec![true_up.x, true_up.y, true_up.z, 0.0],
            vec![-forward.x, -forward.y, -forward.z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ],
    };

    &orientation * &Matrix::translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use crate::math::utils::f64_eq;

    use super::*;

    #[test]
    fn constructing_ray_with_transformed_camera() {
        let transform = &Matrix::rotation_y(PI / 4.0) * &Matrix::translation(0.0, -2.0, 5.0);
        let c = Camera::new_with_transform(201, 101, PI / 2.0, transform);
        let r = c.ray_for_pixel(100, 50);

        let expected = Ray::new(
            Tuple::point(0.0, 2.0, -5.0),
            Tuple::vector((2.0_f64).sqrt() / 2.0, 0.0, (2.0_f64).sqrt() / -2.0),
        );
        assert_eq!(r, expected);
    }

    #[test]
    fn pixel_size_calculated_correctly_horizontal() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(f64_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_calculated_correctly_vertical() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(f64_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn default_orientation_transform() {
        let m = view_transform(
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::point(0.0, 0.0, -1.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );
        assert_eq!(m, Matrix::identity(4));
    }

    #[test]
    fn view_transform_positive_z() {
        // flips the z, i.e. like looking in a mirror it flips the scene
        let m = view_transform(
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::point(0.0, 0.0, 1.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );
        assert_eq!(m, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transform_moves_world() {
        // moves the world back 8 units
        let m = view_transform(
            Tuple::point(0.0, 0.0, 8.0),
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        );
        assert_eq!(m, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transform() {
        let m = view_transform(
            Tuple::point(1.0, 3.0, 2.0),
            Tuple::point(4.0, -2.0, 8.0),
            Tuple::vector(1.0, 1.0, 0.0),
        );
        assert_eq!(
            m,
            Matrix {
                size: 4,
                matrix: vec![
                    vec![-0.50709, 0.50709, 0.67612, -2.36643],
                    vec![0.76772, 0.60609, 0.12122, -2.82843],
                    vec![-0.35857, 0.59761, -0.71714, 0.00000],
                    vec![0.00000, 0.00000, 0.00000, 1.00000]
                ]
            }
        );
    }
}
