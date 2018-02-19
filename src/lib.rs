
pub struct Vertices;
pub struct Indices;

pub struct Mesh(Vertices, Indices);
pub struct Vec3(f32, f32, f32);
pub struct Bounds((f32, f32, f32), (f32, f32, f32));

pub trait GeomField {
    fn f(&self, x : f32, y : f32, z : f32) -> f32;
}

struct SphereField(f32);
impl SphereField {
    pub fn new(r : f32) -> SphereField {
        SphereField(r)
    }
}
impl GeomField for SphereField {
    fn f(&self, x : f32, y : f32, z : f32) -> f32 {
        let &SphereField(r) = self;
        x*x + y*y + z*z - r*r
    }
}

pub fn create_mesh(field : &GeomField, bounds : &Bounds, grid_size : &Vec3) -> Mesh {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sfield = SphereField::new(10.0);
        let mesh = create_mesh(
            &sfield,
            &Bounds((-10.0, -10.0, -10.0), (10.0, 10.0, 10.0)),
            &Vec3(1.0, 1.0, 1.0)
        );
    }
}
