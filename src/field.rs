
pub trait GeomField {
    fn f(&self, x: f32, y: f32, z: f32) -> f32;
}

pub struct Field(Vec<Vec<Vec<f32>>>);

impl Field {

    pub fn from_vecs( vecs : Vec<Vec<Vec<f32>>> ) -> Field {
        Field(vecs)
    }

    pub fn from_geomfield(
        field: &GeomField,
        min_bound: &(f32, f32, f32),
        max_bound: &(f32, f32, f32),
        cube_count: &(usize, usize, usize),
    ) -> Field {
        Self::from_closure(|x,y,z| field.f(x,y,z), min_bound, max_bound, cube_count)
    }

    pub fn from_closure<F>(
        field: F,
        min_bound: &(f32, f32, f32),
        max_bound: &(f32, f32, f32),
        cube_count: &(usize, usize, usize),
    ) -> Field
    where F : Fn(f32,f32,f32) -> f32
    {
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
                    row.push(field(fp.0, fp.1, fp.2));
                }
                slice.push(row)
            }
            field_table.push(slice);
        }
        Field(field_table)
    }

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

