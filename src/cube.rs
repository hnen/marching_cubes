
use mesh::Triangle;
use mesh::Vertex;
use mesh::Mesh;
use field::GeomField;

use tables::VERTS_INSIDE_TO_EDGE_ISECT;
use tables::EDGE_ISECTS_TO_TRIS;
use tables::EDGES;

pub fn tessellate_cube(min: &(f32, f32, f32), max: &(f32, f32, f32), field: &GeomField) -> Mesh {
    let p = [
        (min.0, min.1, min.2),
        (max.0, min.1, min.2),
        (max.0, min.1, max.2),
        (min.0, min.1, max.2),
        (min.0, max.1, min.2),
        (max.0, max.1, min.2),
        (max.0, max.1, max.2),
        (min.0, max.1, max.2),
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
    tessellate_corners(&p, &f)
}

pub fn tessellate_corners(p: &[(f32, f32, f32)], f: &[f32]) -> Mesh {
    let corners_in = (0..8).filter(|i| f[*i as usize] < 0.0).fold(
        0,
        |c, i| c | (1 << i),
    );
    let edges = VERTS_INSIDE_TO_EDGE_ISECT[corners_in];
    if edges == 0 {
        Mesh::empty()
    } else {
        let vmap = vec![
            edge_intersection_unwrap(edges, 0, p, f),
            edge_intersection_unwrap(edges, 1, p, f),
            edge_intersection_unwrap(edges, 2, p, f),
            edge_intersection_unwrap(edges, 3, p, f),
            edge_intersection_unwrap(edges, 4, p, f),
            edge_intersection_unwrap(edges, 5, p, f),
            edge_intersection_unwrap(edges, 6, p, f),
            edge_intersection_unwrap(edges, 7, p, f),
            edge_intersection_unwrap(edges, 8, p, f),
            edge_intersection_unwrap(edges, 9, p, f),
            edge_intersection_unwrap(edges, 10, p, f),
            edge_intersection_unwrap(edges, 11, p, f),
        ];

        let tri_inds = EDGE_ISECTS_TO_TRIS[corners_in];

        let tris: Vec<_> = tri_inds
            .iter()
            .filter_map(|t| if let Some(t) = *t {
                Some(Triangle(t.0, t.1, t.2))
            } else {
                None
            })
            .collect();

        Mesh::new(vmap, tris)
    }
}

#[inline]
fn edge_intersection_unwrap(edges: usize, i: usize, p: &[(f32, f32, f32)], f: &[f32]) -> Vertex {
    if let Some((x, y, z)) = edge_intersection(edges, i, p, f) {
        Vertex(x, y, z)
    } else {
        Vertex(0.0, 0.0, 0.0)
    }
}

#[inline]
fn edge_intersection(
    edges: usize,
    i: usize,
    p: &[(f32, f32, f32)],
    f: &[f32],
) -> Option<(f32, f32, f32)> {
    if (edges >> i) & 1 == 0 {
        None
    } else {
        let (v0, v1) = EDGES[i];
        let (p0, p1) = (&p[v0], &p[v1]);
        let (f0, f1) = (f[v0], f[v1]);
        if (f0 - f1).abs() < 0.000001 {
            Some(*p0)
        } else {
            Some((
                p0.0 - f0 * (p1.0 - p0.0) / (f1 - f0),
                p0.1 - f0 * (p1.1 - p0.1) / (f1 - f0),
                p0.2 - f0 * (p1.2 - p0.2) / (f1 - f0),
            ))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corners() {
        // an unit cube (0,0,0)-(1,1,1)
        let p = [
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (1.0, 0.0, 1.0),
            (0.0, 0.0, 1.0),

            (0.0, 1.0, 0.0),
            (1.0, 1.0, 0.0),
            (1.0, 1.0, 1.0),
            (0.0, 1.0, 1.0),
        ];

        // Tessellate with isect at YZ plane in middle of the cube
        let m = tessellate_corners(&p, &[-1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0]);
        assert_eq!(2, m.1.len()); // tessellates with quad, so 2 triangles

        let mut area = None;

        // Check that normals face towards +X
        for t in m.1 {
            let v0 = &m.0[t.0];
            let v1 = &m.0[t.1];
            let v2 = &m.0[t.2];
            let e0 = (v1.0 - v0.0, v1.1 - v0.1, v1.2 - v0.2);
            let e1 = (v2.0 - v0.0, v2.1 - v0.1, v2.2 - v0.2);
            let n = (
                e0.1 * e1.2 - e0.2 * e1.1,
                -(e0.0 * e1.2 - e1.0 * e0.2),
                e0.0 * e1.1 - e0.1 * e1.0,
            );
            assert!(n.0 > 0.0);
            assert_eq!(n.1, 0.0);
            assert_eq!(n.2, 0.0);
            if let Some(area) = area {
                // while we're at it, make sure triangles have the same area. this happens to be
                // length of the cross product (divided by 2 but we're doing eq comparison so
                // doesn't matter)
                assert_eq!(n.0, area);
            } else {
                area = Some(n.0);
            }
        }

        // Tessellate with one corner inside volume
        let m = tessellate_corners(&p, &[-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        assert_eq!(1, m.1.len()); // Intersects 3 edges, so 1 triangle

        // Normal should be facing (1,1,1)
        for t in m.1 {
            let v0 = &m.0[t.0];
            let v1 = &m.0[t.1];
            let v2 = &m.0[t.2];
            let e0 = (v1.0 - v0.0, v1.1 - v0.1, v1.2 - v0.2);
            let e1 = (v2.0 - v0.0, v2.1 - v0.1, v2.2 - v0.2);
            let n = (
                e0.1 * e1.2 - e0.2 * e1.1,
                -(e0.0 * e1.2 - e1.0 * e0.2),
                e0.0 * e1.1 - e0.1 * e1.0,
            );
            assert_eq!(n.0, n.1);
            assert_eq!(n.0, n.2);
            assert!(n.0 > 0.0)
        }

    }

    #[test]
    fn test_edge_isect() {
        // an unit cube (0,0,0)-(1,1,1)
        let p = [
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (1.0, 0.0, 1.0),
            (0.0, 0.0, 1.0),

            (0.0, 1.0, 0.0),
            (1.0, 1.0, 0.0),
            (1.0, 1.0, 1.0),
            (0.0, 1.0, 1.0),
        ];

        // Intersection is on YZ plane in middle of the unit cube.
        // Test that intersection is found in the middle of the edge (and correct edge is returned.)
        assert_eq!(
            edge_intersection(1, 0, &p, &[-1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0]),
            Some((0.5, 0.0, 0.0))
        );
        assert_eq!(
            edge_intersection(0x3ff, 2, &p, &[-1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0]),
            Some((0.5, 0.0, 1.0))
        );
        assert_eq!(
            edge_intersection(0x3ff, 4, &p, &[-1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0]),
            Some((0.5, 1.0, 0.0))
        );
        assert_eq!(
            edge_intersection(0x3ff, 6, &p, &[-1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0]),
            Some((0.5, 1.0, 1.0))
        );

        // Intersection is on XY plane in middle of unit cube
        assert_eq!(
            edge_intersection(0x3ff, 1, &p, &[-1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0]),
            Some((1.0, 0.0, 0.5))
        );
        assert_eq!(
            edge_intersection(0x3ff, 3, &p, &[-1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0]),
            Some((0.0, 0.0, 0.5))
        );
        assert_eq!(
            edge_intersection(0x3ff, 5, &p, &[-1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0]),
            Some((1.0, 1.0, 0.5))
        );
        assert_eq!(
            edge_intersection(0x3ff, 7, &p, &[-1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0]),
            Some((0.0, 1.0, 0.5))
        );

        // Intersection is on XZ plane in the middle of unit cube
        assert_eq!(
            edge_intersection(0x3ff, 8, &p, &[-1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0]),
            Some((0.0, 0.5, 0.0))
        );
        assert_eq!(
            edge_intersection(0x3ff, 9, &p, &[-1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0]),
            Some((1.0, 0.5, 0.0))
        );
        assert_eq!(
            edge_intersection(0xfff, 10, &p, &[-1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0]),
            Some((1.0, 0.5, 1.0))
        );
        assert_eq!(
            edge_intersection(0xfff, 11, &p, &[-1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0]),
            Some((0.0, 0.5, 1.0))
        );

    }


}
