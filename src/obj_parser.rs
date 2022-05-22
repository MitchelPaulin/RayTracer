use crate::draw::material::Material;
use crate::math::matrix::Matrix;
use crate::math::tuples::Tuple;
use crate::shapes::group::Group;
use crate::shapes::intersect::Intersectable;
use crate::shapes::smooth_triangle::SmoothTriangle;
use crate::shapes::triangle::Triangle;

pub fn parse_obj_file(s: &str, transform: Option<Matrix>, material: Option<Material>) -> Group {
    let mut group = Group::new(transform, material);

    // obj files are 1-indexed so add a dummy vector to shift all data over by 1
    let mut vertices: Vec<Tuple> = vec![Tuple::vector(0.0, 0.0, 0.0)];
    let mut normals: Vec<Tuple> = vec![Tuple::vector(0.0, 0.0, 0.0)];

    for line in s.lines() {
        let symbols: Vec<&str> = line
            .split(' ')
            .filter(|x| !x.contains(char::is_whitespace) && !x.is_empty())
            .collect();

        if symbols.is_empty() {
            continue;
        }

        match symbols[0] {
            "v" => vertices.push(Tuple::point(
                symbols[1].parse::<f64>().unwrap(),
                symbols[2].parse::<f64>().unwrap(),
                symbols[3].parse::<f64>().unwrap(),
            )),
            "vn" => normals.push(Tuple::vector(
                symbols[1].parse::<f64>().unwrap(),
                symbols[2].parse::<f64>().unwrap(),
                symbols[3].parse::<f64>().unwrap(),
            )),
            "f" => {
                let mut face_vertices_indices = vec![];
                let mut face_normal_indices = vec![];
                for symbol in symbols.iter().skip(1) {
                    let face_info: Vec<&str> = symbol.split('/').collect();
                    face_vertices_indices.push(face_info[0].parse::<usize>().unwrap());
                    face_normal_indices.push(if face_info.len() >= 2 {
                        match face_info[2].parse::<usize>() {
                            Ok(i) => Some(i),
                            Err(_) => None,
                        }
                    } else {
                        None
                    })
                }
                for t in fan_triangulation(
                    face_vertices_indices,
                    face_normal_indices,
                    &vertices,
                    &normals,
                ) {
                    group.add_object(t);
                }
            }
            _ => {
                // ignore unrecognized lines
            }
        }
    }

    group
}

// convert a face into a set of triangles
fn fan_triangulation(
    vector_indices: Vec<usize>,
    normal_indices: Vec<Option<usize>>,
    vertices: &[Tuple],
    normals: &[Tuple],
) -> Vec<Box<dyn Intersectable>> {
    let mut triangles: Vec<Box<dyn Intersectable>> = vec![];

    for i in 1..vector_indices.len() - 1 {
        triangles.push(match normal_indices[i] {
            Some(_) => Box::new(SmoothTriangle::new(
                vertices[vector_indices[0]],
                vertices[vector_indices[i]],
                vertices[vector_indices[i + 1]],
                normals[normal_indices[0].unwrap()],
                normals[normal_indices[i].unwrap()],
                normals[normal_indices[i + 1].unwrap()],
                None,
            )),
            None => Box::new(Triangle::new(
                vertices[vector_indices[0]],
                vertices[vector_indices[i]],
                vertices[vector_indices[i + 1]],
                None,
            )),
        });
    }

    triangles
}

#[cfg(test)]
mod test {
    use super::parse_obj_file;

    #[test]
    fn triangles_made() {
        let data = "
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        f 1 2 3
        f 1 3 4";

        let g = parse_obj_file(data, None, None);
        assert_eq!(g.objects.len(), 2);
    }

    #[test]
    fn triangulation_of_polygons() {
        let data = "
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        v 0 2 0
        f 1 2 3 4 5";

        let g = parse_obj_file(data, None, None);
        assert_eq!(g.objects.len(), 3);
    }
}
