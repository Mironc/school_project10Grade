use std::path::Path;
use graphics::objects::{model::{Model,from_str}, vertex::ModelVertex};

use crate::{AssetError, Assets};

pub trait Modelmporter {
    fn import_model<P: AsRef<Path>>(&mut self, path: P) -> Result<Model<ModelVertex>,AssetError>;
}
impl Modelmporter for Assets {
    fn import_model<P: AsRef<Path>>(&mut self, path: P) -> Result<Model<ModelVertex>,AssetError> {
        let source = self.open_file_string(path)?;
        from_str(&source).ok_or(AssetError::AssetImportingError)
    }
}
