use crate::math::tuples::Tuple;
use crate::shapes::group::Group;
use crate::shapes::triangle::Triangle;

pub fn parse_obj_file(s: &str) -> Group {
    let mut group = Group::new(None);
    let mut vertices: Vec<Tuple> = vec![Tuple::vector(0.0, 0.0, 0.0)];

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
            "f" => {
                let mut face_vertices_indices = vec![];
                for i in 1..symbols.len() {
                    let face_info: Vec<&str> = symbols[i].split('/').collect();
                    face_vertices_indices.push(face_info[0].parse::<usize>().unwrap());
                }
                for t in fan_triangulation(face_vertices_indices, &vertices) {
                    group.add_object(Box::new(t));
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
fn fan_triangulation(indices: Vec<usize>, vertices: &Vec<Tuple>) -> Vec<Triangle> {
    let mut triangles = vec![];

    for i in 1..indices.len() - 1 {
        triangles.push(Triangle::new(
            vertices[indices[0]],
            vertices[indices[i]],
            vertices[indices[i + 1]],
            None,
        ));
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

        let g = parse_obj_file(data);
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

        let g = parse_obj_file(data);
        assert_eq!(g.objects.len(), 3);
    }
}
