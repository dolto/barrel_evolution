use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::Indices};

pub fn make_rect_mesh(rects: &Vec<(f32, f32, f32, f32)>, color: Color) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // convert color -> linear rgba array
    let lin = color.to_linear();
    let color_arr = [lin.red, lin.green, lin.blue, lin.alpha];

    for (i, &(cx, cy, w, h)) in rects.iter().enumerate() {
        let hw = w / 2.0;
        let hh = h / 2.0;

        // center-based vertices: bottom-left, bottom-right, top-right, top-left
        let v0 = [cx - hw, cy - hh, 0.0];
        let v1 = [cx + hw, cy - hh, 0.0];
        let v2 = [cx + hw, cy + hh, 0.0];
        let v3 = [cx - hw, cy + hh, 0.0];

        let base = (i * 4) as u32;

        positions.push(v0);
        positions.push(v1);
        positions.push(v2);
        positions.push(v3);

        normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 4]);

        // standard UVs (useful if you later apply a texture)
        uvs.push([0.0, 0.0]);
        uvs.push([1.0, 0.0]);
        uvs.push([1.0, 1.0]);
        uvs.push([0.0, 1.0]);

        // same vertex color for all 4 verts of this rect
        colors.push(color_arr);
        colors.push(color_arr);
        colors.push(color_arr);
        colors.push(color_arr);

        // two triangles (CCW)
        indices.extend_from_slice(&[
            base + 0,
            base + 1,
            base + 2, // (0,1,2)
            base + 0,
            base + 2,
            base + 3, // (0,2,3)
        ]);
    }

    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices)); // or mesh.insert_indices(Indices::U32(indices))

    mesh
}
