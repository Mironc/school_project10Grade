use std::path::Path;

use image::DynamicImage;

use crate::{AssetError, Assets};

pub trait ImageImporter {
    fn import_image<P: AsRef<Path>>(&mut self, path: P) -> Result<DynamicImage, AssetError>;
}
impl ImageImporter for Assets {
    fn import_image<P: AsRef<Path>>(&mut self, path: P) -> Result<DynamicImage, AssetError> {
        Ok(image::load_from_memory_with_format(
            &self.open_file(&path)?,
            image::ImageFormat::from_extension(path.as_ref().extension().ok_or(AssetError::AssetImportingError)?.to_str().ok_or(AssetError::AssetImportingError)?).ok_or(AssetError::AssetImportingError)?,
        )?)
    }
}
