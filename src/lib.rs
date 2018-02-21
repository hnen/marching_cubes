#![allow(unused_variables)]
#![allow(dead_code)]

mod tables;
mod cube;
mod field;
mod mesh;

use field::GeomField;
use mesh::Mesh;
use mesh::Triangle;
use mesh::Vertex;

use cube::tessellate_corners;


struct SphereField(f32);
impl SphereField {
    pub fn new(r: f32) -> SphereField {
        SphereField(r)
    }
}
impl GeomField for SphereField {
    fn f(&self, x: f32, y: f32, z: f32) -> f32 {
        let &SphereField(r) = self;
        x * x + y * y + z * z - r * r
    }
}

struct FieldPrecomputed(Vec<Vec<Vec<f32>>>);
impl FieldPrecomputed {
    pub fn cube_count(&self) -> (usize, usize, usize) {
        (
            self.0[0][0].len() - 1,
            self.0[0].len() - 1,
            self.0.len() - 1,
        )
    }
    pub fn corner_count(&self) -> (usize, usize, usize) {
        (self.0[0][0].len(), self.0[0].len(), self.0.len())
    }
    pub fn f(&self, x: usize, y: usize, z: usize) -> f32 {
        self.0[z][y][x]
    }
}

pub fn create_mesh(
    field: &GeomField,
    min_bound: &(f32, f32, f32),
    max_bound: &(f32, f32, f32),
    cube_count: &(usize, usize, usize),
) -> Mesh {
    let field_table = precompute_field(field, min_bound, max_bound, cube_count);
    create_mesh_precomputed(&field_table, min_bound, max_bound)
}

fn create_mesh_precomputed(
    field: &FieldPrecomputed,
    min_bound: &(f32, f32, f32),
    max_bound: &(f32, f32, f32),
) -> Mesh {
    let cube_size = (
        (max_bound.0 - min_bound.0) / (field.cube_count().0 as f32),
        (max_bound.1 - min_bound.1) / (field.cube_count().1 as f32),
        (max_bound.2 - min_bound.2) / (field.cube_count().2 as f32),
    );
    let mut verts = Vec::new();
    let mut tris = Vec::new();
    for z in 0..field.cube_count().2 {
        for y in 0..field.cube_count().1 {
            for x in 0..field.cube_count().0 {
                let (fx, fy, fz) = (x as f32, y as f32, z as f32);
                let c0 = (
                    min_bound.0 + fx * cube_size.0,
                    min_bound.1 + fy * cube_size.1,
                    min_bound.2 + fz * cube_size.2,
                );
                let c1 = (c0.0 + cube_size.0, c0.1 + cube_size.1, c0.2 + cube_size.2);
                let p = [
                    (c0.0, c0.1, c0.2),
                    (c1.0, c0.1, c0.2),
                    (c1.0, c0.1, c1.2),
                    (c0.0, c0.1, c1.2),
                    (c0.0, c1.1, c0.2),
                    (c1.0, c1.1, c0.2),
                    (c1.0, c1.1, c1.2),
                    (c0.0, c1.1, c1.2),
                ];
                let f = [
                    field.f(x, y, z),
                    field.f(x + 1, y, z),
                    field.f(x + 1, y, z + 1),
                    field.f(x, y, z + 1),
                    field.f(x, y + 1, z),
                    field.f(x + 1, y + 1, z),
                    field.f(x + 1, y + 1, z + 1),
                    field.f(x, y + 1, z + 1),
                ];
                let Mesh(cube_verts, cube_tris) = tessellate_corners(&p, &f);
                for Triangle(i0, i1, i2) in cube_tris {
                    tris.push(Triangle(verts.len(), verts.len() + 1, verts.len() + 2));
                    verts.push(cube_verts[i0].clone());
                    verts.push(cube_verts[i1].clone());
                    verts.push(cube_verts[i2].clone());
                }
            }
        }
    }
    Mesh(verts, tris)
}

fn precompute_field(
    field: &GeomField,
    min_bound: &(f32, f32, f32),
    max_bound: &(f32, f32, f32),
    cube_count: &(usize, usize, usize),
) -> FieldPrecomputed {
    let corner_counts = (cube_count.0 + 1, cube_count.1 + 1, cube_count.2 + 1);
    let mut field_table = Vec::with_capacity(corner_counts.0);
    for z in 0..corner_counts.2 {
        let mut slice = Vec::with_capacity(corner_counts.1);
        for y in 0..corner_counts.1 {
            let mut row = Vec::with_capacity(corner_counts.2);
            for x in 0..corner_counts.0 {
                let (fx, fy, fz) = (x as f32, y as f32, z as f32);
                let fp = (
                    min_bound.0 + fx * (max_bound.0 - min_bound.0) / (cube_count.0 as f32),
                    min_bound.1 + fy * (max_bound.1 - min_bound.1) / (cube_count.1 as f32),
                    min_bound.2 + fz * (max_bound.2 - min_bound.2) / (cube_count.2 as f32),
                );
                row.push(field.f(fp.0, fp.1, fp.2));
            }
            slice.push(row)
        }
        field_table.push(slice);
    }
    FieldPrecomputed(field_table)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_works() {
        let sfield = SphereField::new(10.0);
        let _mesh = create_mesh(&sfield, &(-10.0, -10.0, -10.0), &(10.0, 10.0, 10.0), &(
            20,
            20,
            20,
        ));
    }

    #[test]
    fn test_precomputed() {
        let field = field_precomputed();
        let mesh = create_mesh_precomputed(&field, &(-1.0, -1.0, -1.0), &(1.0, 1.0, 1.0));
        // The result should be with the test field a regular octahedron with
        // bounds at +-(0.5,0.5,0.5)
        assert_eq!(8, mesh.1.len());
        assert_eq!(
            Some(&Vertex(-0.5, 0.0, 0.0)),
            mesh.0.iter().min_by(
                |&&Vertex(x0, y0, z0), &&Vertex(x1, y1, z1)| {
                    x0.partial_cmp(&x1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex( 0.5, 0.0, 0.0)),
            mesh.0.iter().max_by(
                |&&Vertex(x0, y0, z0), &&Vertex(x1, y1, z1)| {
                    x0.partial_cmp(&x1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex( 0.0,-0.5, 0.0)),
            mesh.0.iter().min_by(
                |&&Vertex(x0, y0, z0), &&Vertex(x1, y1, z1)| {
                    y0.partial_cmp(&y1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex( 0.0, 0.5, 0.0)),
            mesh.0.iter().max_by(
                |&&Vertex(x0, y0, z0), &&Vertex(x1, y1, z1)| {
                    y0.partial_cmp(&y1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex( 0.0, 0.0,-0.5)),
            mesh.0.iter().min_by(
                |&&Vertex(x0, y0, z0), &&Vertex(x1, y1, z1)| {
                    z0.partial_cmp(&z1).unwrap()
                },
            )
        );
        assert_eq!(
            Some(&Vertex( 0.0, 0.0, 0.5)),
            mesh.0.iter().max_by(
                |&&Vertex(x0, y0, z0), &&Vertex(x1, y1, z1)| {
                    z0.partial_cmp(&z1).unwrap()
                },
            )
        );
    }

    fn field_precomputed() -> FieldPrecomputed {
        let f = FieldPrecomputed(vec![
            vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
            vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, 1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
            vec![
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
                vec![-1.0, -1.0, -1.0],
            ],
        ]);
        assert_eq!(f.corner_count(), (3, 3, 3));
        assert_eq!(f.cube_count(), (2, 2, 2));
        assert_eq!(f.f(0, 0, 0), -1.0);
        assert_eq!(f.f(1, 0, 0), -1.0);
        assert_eq!(f.f(2, 0, 0), -1.0);
        assert_eq!(f.f(0, 1, 0), -1.0);
        assert_eq!(f.f(1, 1, 0), -1.0);
        assert_eq!(f.f(2, 1, 0), -1.0);
        assert_eq!(f.f(0, 2, 0), -1.0);
        assert_eq!(f.f(1, 2, 0), -1.0);
        assert_eq!(f.f(2, 2, 0), -1.0);

        assert_eq!(f.f(0, 0, 1), -1.0);
        assert_eq!(f.f(1, 0, 1), -1.0);
        assert_eq!(f.f(2, 0, 1), -1.0);
        assert_eq!(f.f(0, 1, 1), -1.0);
        assert_eq!(f.f(1, 1, 1), 1.0);
        assert_eq!(f.f(2, 1, 1), -1.0);
        assert_eq!(f.f(0, 2, 1), -1.0);
        assert_eq!(f.f(1, 2, 1), -1.0);
        assert_eq!(f.f(2, 2, 1), -1.0);

        assert_eq!(f.f(0, 0, 2), -1.0);
        assert_eq!(f.f(1, 0, 2), -1.0);
        assert_eq!(f.f(2, 0, 2), -1.0);
        assert_eq!(f.f(0, 1, 2), -1.0);
        assert_eq!(f.f(1, 1, 2), -1.0);
        assert_eq!(f.f(2, 1, 2), -1.0);
        assert_eq!(f.f(0, 2, 2), -1.0);
        assert_eq!(f.f(1, 2, 2), -1.0);
        assert_eq!(f.f(2, 2, 2), -1.0);

        f
    }


}
