use std::path::Path;

//use crate::{AssetError, Assets};

pub trait PathEXT {
    fn get_name(&self) -> Option<String>;
    fn to_string(&self) -> Option<String>;
}
impl PathEXT for Path {
    fn get_name(&self) -> Option<String> {
        Some(self.file_name().as_ref()?.to_str().as_ref()?.to_string())
    }
    fn to_string(&self) -> Option<String> {
        Some(self.as_os_str().to_str().unwrap().to_owned())
    }
}
/* TODO:For future 
pub trait Importer{
    type T;
    fn load<P:AsRef<Path>>(assets:&mut Assets,path:P) ->Result<Self::T,AssetError>;
}
*/