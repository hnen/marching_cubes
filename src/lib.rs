#![feature(nll)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tables;
mod cube;
mod field;
mod mesh;
mod tessellator;

pub use field::GeomField;
pub use tessellator::Field;

pub use tessellator::create_mesh_from_field;
pub use tessellator::create_mesh_precomputed;
