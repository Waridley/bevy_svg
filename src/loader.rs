use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt,  LoadContext},
    log::debug,
    math::Vec2,
    reflect::{Reflect, ReflectDeserialize, ReflectSerialize},
    transform::components::Transform,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{origin::Origin, svg::Svg};

#[derive(Default)]
pub struct SvgAssetLoader;

impl AssetLoader for SvgAssetLoader {
    type Asset = Svg;
    type Settings = ();
    type Error = FileSvgError;

    async fn load<'load>(
        &'load self,
        reader: &'load mut Reader<'_>,
        settings: &'load Self::Settings,
        load_context: &'load mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        debug!("Parsing SVG: {} ...", load_context.path().display());
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .map_err(|e| FileSvgError {
                error: e.into(),
                path: load_context.path().display().to_string(),
            })?;

        let mut svg = Svg::from_bytes(&bytes, load_context.path(), None::<&std::path::Path>)?;
        let name = &load_context
            .path()
            .file_name()
            .ok_or_else(|| FileSvgError {
                error: SvgError::InvalidFileName(load_context.path().display().to_string()),
                path: load_context.path().display().to_string(),
            })?
            .to_string_lossy();
        svg.name = name.to_string();
        debug!("Parsing SVG: {} ... Done", load_context.path().display());

        Ok(svg)
    }

    fn extensions(&self) -> &[&str] {
        &["svg", "svgz"]
    }
}

/// An error that occurs when loading a texture
#[derive(Error, Debug)]
pub enum SvgError {
    #[error("invalid file name")]
    InvalidFileName(String),
    #[error("could not read file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("failed to load an SVG: {0}")]
    SvgError(#[from] usvg::Error),
}

/// An error that occurs when loading a texture from a file.
#[derive(Error, Debug)]
pub struct FileSvgError {
    pub(crate) error: SvgError,
    pub(crate) path: String,
}
impl std::fmt::Display for FileSvgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Error reading SVG file {}: {}, this is an error in `bevy_svg`.",
            self.path, self.error
        )
    }
}
