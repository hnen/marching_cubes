#![allow(unused_variables)]
#![allow(dead_code)]

mod tables;
mod cube;
mod field;
mod mesh;

use field::GeomField;
use mesh::Mesh;

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

pub fn create_mesh(field: &GeomField, min_bound: &(f32,f32,f32), max_bound: &(f32,f32,f32), grid_size: &(f32,f32,f32)) -> Mesh {
    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_works() {
        let sfield = SphereField::new(10.0);
        let _mesh = create_mesh(
            &sfield,
            &(-10.0, -10.0, -10.0), &(10.0, 10.0, 10.0),
            &(1.0, 1.0, 1.0),
        );
    }


}
