
mod tables;

use tables::VERTS_INSIDE_TO_EDGE_ISECT;
use tables::EDGES;
use tables::EDGE_ISECTS_TO_TRIS;

pub struct Vertices;
pub struct Indices;

pub struct Mesh(Vertices, Indices);
pub struct Vec3(f32, f32, f32);
pub struct Bounds((f32, f32, f32), (f32, f32, f32));

impl Mesh {
    pub fn new(verts : Vec<Vec3>, inds : Vec<usize>) -> Mesh {
        unimplemented!();
    }
    pub fn empty() -> Mesh { unimplemented!(); }
}

pub trait GeomField {
    fn f(&self, x: f32, y: f32, z: f32) -> f32;
}

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

pub fn create_mesh(field: &GeomField, bounds: &Bounds, grid_size: &Vec3) -> Mesh {
    unimplemented!();
}

fn tessellate_cube(min : &Vec3, max : &Vec3, field : &GeomField) -> Mesh {
    let p = [
        Vec3(min.0, min.1, min.2),
        Vec3(max.0, min.1, min.2),
        Vec3(max.0, min.1, max.2),
        Vec3(min.0, min.1, max.2),
        Vec3(min.0, max.1, min.2),
        Vec3(max.0, max.1, min.2),
        Vec3(max.0, max.1, max.2),
        Vec3(min.0, max.1, max.2),
    ];
    let f = [
        field.f(p[0].0, p[0].1, p[0].2),
        field.f(p[1].0, p[1].1, p[1].2),
        field.f(p[2].0, p[2].1, p[2].2),
        field.f(p[3].0, p[3].1, p[3].2),
        field.f(p[4].0, p[4].1, p[4].2),
        field.f(p[5].0, p[5].1, p[5].2),
        field.f(p[6].0, p[6].1, p[6].2),
        field.f(p[7].0, p[7].1, p[7].2),
    ];
    tessellate_corners(&p,&f)
}

fn tessellate_corners(p : &[Vec3], f : &[f32]) -> Mesh {
    let corners_in = (0..8).filter(|i| f[*i as usize] < 0.0).fold(0, |c, i| c | (1<<i));
    let edges = VERTS_INSIDE_TO_EDGE_ISECT[corners_in];
    if edges == 0 {
        Mesh::empty()
    } else {
        let vmap = [
            edge_intersection(edges, 0, p, f), edge_intersection(edges, 1, p, f),
            edge_intersection(edges, 2, p, f), edge_intersection(edges, 3, p, f),
            edge_intersection(edges, 4, p, f), edge_intersection(edges, 5, p, f),
            edge_intersection(edges, 6, p, f), edge_intersection(edges, 7, p, f),
            edge_intersection(edges, 8, p, f), edge_intersection(edges, 9, p, f),
            edge_intersection(edges, 10, p, f), edge_intersection(edges, 11, p, f),
        ];

        let mut inds = Vec::with_capacity(15);
        let mut verts = Vec::with_capacity(15);

        for t in EDGE_ISECTS_TO_TRIS[edges].iter() {
            let t = t.unwrap();
            inds.push(verts.len()+0);
            inds.push(verts.len()+1);
            inds.push(verts.len()+2);
            let v0 = vmap[t.0].unwrap();
            let v1 = vmap[t.1].unwrap();
            let v2 = vmap[t.2].unwrap();
            verts.push(Vec3(v0.0,v0.1,v0.2));
            verts.push(Vec3(v1.0,v1.1,v1.2));
            verts.push(Vec3(v2.0,v2.1,v2.2));
        }

        Mesh::new(verts, inds)
    }
}

#[inline]
fn edge_intersection(edges : usize, i : usize, p : &[Vec3], f : &[f32]) -> Option<(f32, f32, f32)> {
    if (edges >> i) & 1 == 0 {
        None
    } else {
        let (v0, v1) = EDGES[i];
        let (ref p0, ref p1) = (&p[v0], &p[v1 ]);
        let (f0, f1) = (f[v0], f[v1]);
        Some( (
            p0.0 - f0 * (p1.0 - p0.0) / (f1 - f0),
            p0.1 - f0 * (p1.1 - p0.1) / (f1 - f0),
            p0.2 - f0 * (p1.2 - p0.2) / (f1 - f0)
        ) )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[should_panic]
    fn it_works() {
        let sfield = SphereField::new(10.0);
        let _mesh = create_mesh(
            &sfield,
            &Bounds((-10.0, -10.0, -10.0), (10.0, 10.0, 10.0)),
            &Vec3(1.0, 1.0, 1.0),
        );
    }

    #[test]
    fn test_edge_isect() {
        let p = [
            Vec3(0.0,0.0,0.0),
            Vec3(1.0,0.0,0.0),
            Vec3(1.0,0.0,1.0),
            Vec3(0.0,0.0,1.0),

            Vec3(0.0,1.0,0.0),
            Vec3(1.0,1.0,0.0),
            Vec3(1.0,1.0,1.0),
            Vec3(0.0,1.0,1.0),
        ];

        // Intersection is on YZ plane in middle of the unit cube
        assert_eq!( edge_intersection(1, 0, &p, &[
            -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0,
        ]), Some((0.5, 0.0, 0.0)) );
        assert_eq!( edge_intersection(0x3ff, 2, &p, &[
            -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0,
        ]), Some((0.5, 0.0, 1.0)) );
        assert_eq!( edge_intersection(0x3ff, 4, &p, &[
            -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0,
        ]), Some((0.5, 1.0, 0.0)) );
        assert_eq!( edge_intersection(0x3ff, 6, &p, &[
            -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0,
        ]), Some((0.5, 1.0, 1.0)) );

        // Intersection is on XY plane in middle of unit cube
        assert_eq!( edge_intersection(0x3ff, 1, &p, &[
            -1.0,-1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0, 1.0,
        ]), Some((1.0, 0.0, 0.5)) );
        assert_eq!( edge_intersection(0x3ff, 3, &p, &[
            -1.0,-1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0, 1.0,
        ]), Some((0.0, 0.0, 0.5)) );
        assert_eq!( edge_intersection(0x3ff, 5, &p, &[
            -1.0,-1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0, 1.0,
        ]), Some((1.0, 1.0, 0.5)) );
        assert_eq!( edge_intersection(0x3ff, 7, &p, &[
            -1.0,-1.0, 1.0, 1.0,
            -1.0,-1.0, 1.0, 1.0,
        ]), Some((0.0, 1.0, 0.5)) );

        // Intersection is on XZ plane in the middle of unit cube
        assert_eq!( edge_intersection(0x3ff, 8, &p, &[
            -1.0,-1.0,-1.0,-1.0,
             1.0, 1.0, 1.0, 1.0,
        ]), Some((0.0, 0.5, 0.0)) );
        assert_eq!( edge_intersection(0x3ff, 9, &p, &[
            -1.0,-1.0,-1.0,-1.0,
            1.0, 1.0, 1.0, 1.0,
        ]), Some((1.0, 0.5, 0.0)) );
        assert_eq!( edge_intersection(0xfff, 10, &p, &[
            -1.0,-1.0,-1.0,-1.0,
            1.0, 1.0, 1.0, 1.0,
        ]), Some((1.0, 0.5, 1.0)) );
        assert_eq!( edge_intersection(0xfff, 11, &p, &[
            -1.0,-1.0,-1.0,-1.0,
            1.0, 1.0, 1.0, 1.0,
        ]), Some((0.0, 0.5, 1.0)) );

    }

}
