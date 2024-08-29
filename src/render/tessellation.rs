use bevy::{
    log::{debug, error},
    math::Vec3,
    transform::components::Transform,
};
use bevy::math::Vec2;
use lyon_tessellation::{BuffersBuilder, FillOptions, FillTessellator, StrokeTessellator};

use crate::{
    render::{SvgMesh3d, vertex_buffer::{BufferExt, IndexType, Vertex, VertexBuffers, VertexConstructor}},
    svg::{DrawType, Svg},
};

pub(crate) fn generate_buffer(
    svg: &Svg,
    fill_tess: &mut FillTessellator,
    stroke_tess: &mut StrokeTessellator,
    settings: &SvgMesh3d,
) -> VertexBuffers {
    debug!("Tessellating SVG: {}", svg.name);

    let front_z = {
        #[cfg(feature = "3d")]
        {
            Vec3::Z * settings.depth.unwrap_or(0.0) * 0.5
        }
        #[cfg(not(feature = "3d"))]
        Vec3::ZERO
    };
    let size = settings.size.unwrap_or(svg.size);
    let scale = settings.size.map(|size| size / svg.size)
      .unwrap_or(Vec2::ONE);
    let transform = Transform {
        translation: settings.rotation * (settings.origin.compute_translation(size) + front_z),
        rotation: settings.rotation,
        // Bevy has a different y-axis origin, so we need to flip that axis
        scale: Vec3::new(scale.x, -scale.y, 1.0),
    };

    let mut buffers = VertexBuffers::new();

    for path in &svg.paths {
        let mut buffer = VertexBuffers::new();

        let transform = transform * path.abs_transform;
        let mut builder = BuffersBuilder::new(
            &mut buffer,
            VertexConstructor {
                color: path.color,
                transform,
            },
        );
        match path.draw_type {
            DrawType::Fill => {
                if let Err(e) = fill_tess.tessellate(
                    path.segments.clone(),
                    &FillOptions::tolerance(settings.tolerance),
                    &mut builder,
                ) {
                    error!("FillTessellator error: {:?}", e)
                }
            }
            DrawType::Stroke(opts) => {
                if let Err(e) = stroke_tess.tessellate(path.segments.clone(), &opts, &mut builder) {
                    error!("StrokeTessellator error: {:?}", e)
                }
            }
        }
        #[cfg(feature = "3d")]
        if let Some(depth) = settings.depth {
            let offset = (transform.rotation * (Vec3::NEG_Z * (depth * 0.5)));
            let vertices = buffer
                .vertices
                .iter()
                .copied()
                .map(|vert| {
                    let (pos, color) = (vert.position, vert.color);
                    let position = (Vec3::from_array(pos) + offset).to_array();
                    Vertex { position, color }
                })
                .collect();
            let mut indices = Vec::with_capacity(buffer.indices.len());
            indices.extend(
                buffer
                    .indices
                    .chunks(3)
                    .map(|tri| <[IndexType; 3]>::try_from(tri).unwrap())
                    // Reverse winding order for back
                    .flat_map(|[a, b, c]| [c, b, a]),
            );
            let back_buffer = VertexBuffers { vertices, indices };

            // Prepare to find edges so we can add side triangles without infilling.
            let first_back_idx = IndexType::try_from(buffer.vertices.len())
                .expect("vertex indices should fit in `IndexType`");
            let segments = buffer
                .indices
                .chunks(3)
                .map(|tri| <[IndexType; 3]>::try_from(tri).unwrap())
                .flat_map(|[a, b, c]| [[a, b], [b, c], [c, a]])
                .collect::<Vec<_>>();

            buffer.extend_one(back_buffer);

            // Add sides
            for (i, [a, b]) in segments.iter().copied().enumerate() {
                let left = segments[0..i].iter();
                let right = segments.iter().skip(i + 1);
                if !left
                    .chain(right)
                    .any(|other| *other == [a, b] || *other == [b, a])
                {
                    // Outside edge
                    let [c, d] = [a + first_back_idx, b + first_back_idx];
                    buffer.indices.extend_from_slice(&[a, c, b, b, c, d])
                } // else edge is shared with another triangle
            }
        }

        buffers.extend_one(buffer);
    }
    debug!("Tessellating SVG: {} ... Done", svg.name);

    buffers
}
