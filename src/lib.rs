#![allow(unused_variables)]
#![allow(dead_code)]

mod tables;
mod cube;
mod field;
mod mesh;

use field::GeomField;
use mesh::Mesh;

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

pub fn create_mesh(
    field: &GeomField,
    min_bound: &(f32, f32, f32),
    max_bound: &(f32, f32, f32),
    cube_count: &(usize, usize, usize),
) -> Mesh {
    let field_table = precompute_field(field, min_bound, max_bound, cube_count);

    create_mesh_precomputed(&field_table, min_bound, max_bound)

}

fn create_mesh_precomputed(field : &FieldPrecomputed,
   min_bound: &(f32, f32, f32),
   max_bound: &(f32, f32, f32)) -> Mesh {
    unimplemented!();
}

fn precompute_field(
    field: &GeomField,
    min_bound: &(f32, f32, f32),
    max_bound: &(f32, f32, f32),
    cube_count: &(usize, usize, usize),
) -> FieldPrecomputed {
    let corner_counts = (
        cube_count.0 + 1,
        cube_count.1 + 1,
        cube_count.2 + 1,
    );
    let mut field_table = Vec::with_capacity(corner_counts.0);
    for z in 0..corner_counts.2 {
        let mut slice = Vec::with_capacity(corner_counts.1);
        for y in 0..corner_counts.1 {
            let mut row = Vec::with_capacity(corner_counts.2);
            for x in 0..corner_counts.2 {
                let fx = (
                    min_bound.0 + (x as f32) * (max_bound.0 - min_bound.0) / (cube_count.0 as f32),
                    min_bound.1 + (y as f32) * (max_bound.1 - min_bound.1) / (cube_count.1 as f32),
                    min_bound.2 + (z as f32) * (max_bound.2 - min_bound.2) / (cube_count.2 as f32),
                );
                row.push(field.f(fx.0, fx.1, fx.2));
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
        let _mesh = create_mesh(&sfield, &(-10.0, -10.0, -10.0), &(10.0, 10.0, 10.0), &(20,20,20));
    }


}
