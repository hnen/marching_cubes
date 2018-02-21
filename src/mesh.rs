
#[derive(Clone, PartialEq, Debug)] pub struct Vertex(pub f32, pub f32, pub f32);
#[derive(Clone, PartialEq, Debug)] pub struct Triangle(pub usize, pub usize, pub usize);

pub struct Mesh(pub Vec<Vertex>, pub Vec<Triangle>);

impl Mesh {
    pub fn new(verts: Vec<Vertex>, tris: Vec<Triangle>) -> Mesh {
        Mesh(verts, tris)
    }
    pub fn empty() -> Mesh {
        Mesh(Vec::new(), Vec::new())
    }
}
