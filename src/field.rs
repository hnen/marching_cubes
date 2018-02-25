
pub trait GeomField {
    fn f(&self, x: f32, y: f32, z: f32) -> f32;
}

pub struct Field(pub Vec<Vec<Vec<f32>>>);

impl Field {
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
