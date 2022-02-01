use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

pub struct SkyboxShape;

impl From<SkyboxShape> for Mesh {
    fn from(_: SkyboxShape) -> Self {
        let vertices = &[
            // Top
            ([-0.5, -0.5, 0.5], [0., 0., -1.0], [0., 0.]),
            ([0.5, -0.5, 0.5], [0., 0., -1.0], [1.0, 0.]),
            ([0.5, 0.5, 0.5], [0., 0., -1.0], [1.0, 1.0]),
            ([-0.5, 0.5, 0.5], [0., 0., -1.0], [0., 1.0]),
            // Bottom
            ([-0.5, 0.5, -0.5], [0., 0., 1.0], [1.0, 0.]),
            ([0.5, 0.5, -0.5], [0., 0., 1.0], [0., 0.]),
            ([0.5, -0.5, -0.5], [0., 0., 1.0], [0., 1.0]),
            ([-0.5, -0.5, -0.5], [0., 0., 1.0], [1.0, 1.0]),
            // Right
            ([0.5, -0.5, -0.5], [-1.0, 0., 0.], [0., 0.]),
            ([0.5, 0.5, -0.5], [-1.0, 0., 0.], [1.0, 0.]),
            ([0.5, 0.5, 0.5], [-1.0, 0., 0.], [1.0, 1.0]),
            ([0.5, -0.5, 0.5], [-1.0, 0., 0.], [0., 1.0]),
            // Left
            ([-0.5, -0.5, 0.5], [1.0, 0., 0.], [1.0, 0.]),
            ([-0.5, 0.5, 0.5], [1.0, 0., 0.], [0., 0.]),
            ([-0.5, 0.5, -0.5], [1.0, 0., 0.], [0., 1.0]),
            ([-0.5, -0.5, -0.5], [1.0, 0., 0.], [1.0, 1.0]),
            // Front
            ([0.5, 0.5, -0.5], [0., -1.0, 0.], [1.0, 0.]),
            ([-0.5, 0.5, -0.5], [0., -1.0, 0.], [0., 0.]),
            ([-0.5, 0.5, 0.5], [0., -1.0, 0.], [0., 1.0]),
            ([0.5, 0.5, 0.5], [0., -1.0, 0.], [1.0, 1.0]),
            // Back
            ([0.5, -0.5, 0.5], [0., 1.0, 0.], [0., 0.]),
            ([-0.5, -0.5, 0.5], [0., 1.0, 0.], [1.0, 0.]),
            ([-0.5, -0.5, -0.5], [0., 1.0, 0.], [1.0, 1.0]),
            ([0.5, -0.5, -0.5], [0., 1.0, 0.], [0., 1.0]),
        ];

        let mut positions = Vec::with_capacity(24);
        let mut normals = Vec::with_capacity(24);
        let mut uvs = Vec::with_capacity(24);

        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let indices = Indices::U32(vec![
            0, 2, 1, 2, 0, 3, // top
            4, 6, 5, 6, 4, 7, // bottom
            8, 10, 9, 10, 8, 11, // right
            12, 14, 13, 14, 12, 15, // left
            16, 18, 17, 18, 16, 19, // front
            20, 22, 21, 22, 20, 23, // back
        ]);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}
