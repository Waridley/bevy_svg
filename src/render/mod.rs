mod plugin;
pub(crate) mod tessellation;
mod vertex_buffer;

#[cfg(feature = "2d")]
mod svg2d;
#[cfg(feature = "3d")]
mod svg3d;

use bevy::prelude::*;
use bevy::utils::HashMap;
#[cfg(feature = "2d")]
pub use svg2d::{SvgMesh2dBundle, SvgMesh2d};
#[cfg(feature = "3d")]
pub use svg3d::{SvgMesh3dBundle};

pub use plugin::SvgPlugin;
use crate::origin::Origin;
use crate::svg::Svg;

#[derive(Resource, Default, Debug, Deref, DerefMut, Reflect)]
pub struct SvgMeshCache(HashMap<Handle<Svg>, HashMap<SvgMeshKey, Handle<Mesh>>>);

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq, Reflect)]
pub struct SvgMeshKey {
	origin: Origin,
	size: Option<[u32; 2]>,
	depth: Option<u32>,
	rotation: [u32; 4],
	tolerance: u32,
}

impl From<SvgMesh2d> for SvgMeshKey {
	fn from(value: SvgMesh2d) -> Self {
		Self::from(SvgMesh3d::from(value))
	}
}

impl From<SvgMesh3d> for SvgMeshKey {
	fn from(value: SvgMesh3d) -> Self {
		Self {
			origin: value.origin,
			size: value.size.map(|size| [size.x.to_bits(), size.y.to_bits()]),
			depth: value.depth.map(f32::to_bits),
			rotation: [
				value.rotation.x.to_bits(),
				value.rotation.y.to_bits(),
				value.rotation.z.to_bits(),
				value.rotation.w.to_bits(),
			],
			tolerance: value.tolerance.to_bits(),
		}
	}
}

impl From<SvgMeshKey> for SvgMesh2d {
	fn from(value: SvgMeshKey) -> Self {
		#[cfg(debug_assertions)]
		if let Some(depth) = value.depth {
			let depth = f32::from_bits(depth);
			warn!(?depth, "Discarding depth when convert `SvgMeshKey` to `SvgMesh2d`");
		}
		let mesh_3d = SvgMesh3d::from(value);
		Self {
			origin: mesh_3d.origin,
			size: mesh_3d.size,
			rotation: mesh_3d.rotation.to_euler(EulerRot::ZYX).0,
			tolerance: mesh_3d.tolerance,
		}
	}
}

impl From<SvgMeshKey> for SvgMesh3d {
	fn from(value: SvgMeshKey) -> Self {
		Self {
			origin: value.origin,
			size: value.size.map(|size| Vec2::new(
				f32::from_bits(size[0]),
				f32::from_bits(size[1]),
			)),
			depth: value.depth.map(f32::from_bits),
			rotation: Quat::from_xyzw(
				f32::from_bits(value.rotation[0]),
				f32::from_bits(value.rotation[1]),
				f32::from_bits(value.rotation[2]),
				f32::from_bits(value.rotation[3]),
			),
			tolerance: f32::from_bits(value.tolerance),
		}
	}
}

#[derive(Debug, Clone, Component)]
pub struct SvgMesh3d {
    pub origin: Origin,
		pub size: Option<Vec2>,
    pub depth: Option<f32>,
	  pub rotation: Quat,
	  pub tolerance: f32,
}

impl Default for SvgMesh3d {
	fn default() -> Self {
		Self {
			origin: default(),
			size: None,
			depth: None,
			rotation: default(),
			tolerance: 0.001,
		}
	}
}