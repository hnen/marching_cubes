# marching_cubes

Implementation of Marching Cubes algorithm in Rust. The algorithm creates a polygonisation for an isosurface of an arbitrary field.

## Usage

The library requires nightly Rust due to library's usage of non lexical lifetimes (`feature(nll)`). Changing algorithms to be compatible with stable Rust should be a trivial task, but would make some parts of the code a bit more awkward.

Algorithm is invoked by calling `create_mesh(field: &Field, min_bound: &(f32, f32, f32), max_bound: &(f32, f32, f32))`. The function returns a `Mesh`, which is a simple tuple struct for list of vertices and triangle indices. Field can be constructed from an arbitrary closure, which maps an `f32` triplet to a scalar, or from precomputed three dimensional array of `f32`s.

## Acknowledgements

Main reference for the implementation has been Paul Borke's 1994 article "[Polygonising a scalar field](http://paulbourke.net/geometry/polygonise/)"


