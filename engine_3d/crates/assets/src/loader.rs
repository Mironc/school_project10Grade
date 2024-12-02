use crate::ext::PathEXT;
//use crate::ranged_reader::{self, RangedReader};
use byteorder::{ReadBytesExt, BE};
use image::ImageError;
use positioned_io::*;
use std::io::Read;
use std::path::Path;
use thiserror::Error;
#[derive(Debug, Clone)]
pub struct File {
    name: String,
    start: u64,
    end: u64,
}
impl File {
    pub fn new(name: String, start: u64, end: u64) -> Self {
        Self { name, start, end }
    }
}
#[derive(Debug, Clone)]
pub struct Directory {
    path: String,
    files: Vec<File>,
}
impl Directory {
    pub fn new(path: String, files: Vec<File>) -> Self {
        Self { path, files }
    }
}
#[derive(Error, Debug)]
pub enum AssetError {
    IOError(#[from] std::io::Error),
    AssetImportingError,
    FileFindingError,
    NotAFile,
    StringCastingError(#[from] std::string::FromUtf8Error),
    ImageError(#[from] ImageError),
}
impl std::fmt::Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetError::IOError(error) => write!(f, "{}", error),
            AssetError::FileFindingError => write!(
                f,
                "File is missing or some error in (file path/file name) occured"
            ),
            AssetError::NotAFile => write!(f, "found dictionary or symlink instead of file"),
            AssetError::StringCastingError(error) => write!(f, "{}", error),
            AssetError::ImageError(error) => write!(f, "{}", error),
            AssetError::AssetImportingError => write!(f,"had error while loading asset"),
        }
    }
}
use zstd::*;
#[derive(Debug)]
pub struct Assets {
    directories: Vec<Directory>,
    source: RandomAccessFile,
}
impl Assets {
    pub fn new<P: AsRef<Path>>(directories: Vec<Directory>, source: RandomAccessFile) -> Self {
        Self {
            directories,
            source,
        }
    }
    pub fn find_file<P: AsRef<Path>>(&self, path: P) -> Result<&File, AssetError> {
        let directory_path = path.as_ref().parent().ok_or(AssetError::FileFindingError)?;
        let file_name = path.as_ref().get_name().ok_or(AssetError::NotAFile)?;
        let directory = self
            .directories
            .iter()
            .find(|x| x.path == directory_path.to_str().unwrap())
            .ok_or(AssetError::FileFindingError)?;
        let file = directory
            .files
            .iter()
            .find(|x| x.name == file_name)
            .ok_or(AssetError::FileFindingError)?;
        Ok(file)
    }
    pub fn open_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, AssetError> {
        let file = self.find_file(path)?;
        let mut file_buffer = vec![0u8; (file.end - file.start) as usize];
        let _ = self
            .source
            .read_exact_at(file.start, file_buffer.as_mut_slice())?;
        Ok(decode_all(file_buffer.as_slice())?)
    }
    pub fn open_file_string<P: AsRef<Path>>(&self, path: P) -> Result<String, AssetError> {
        let file = self.open_file(path)?;
        Ok(String::from_utf8(file)?)
    }
    pub fn from_data_file<P: AsRef<Path>>(path: P) -> std::io::Result<Assets> {
        let mut data_file = Cursor::new(RandomAccessFile::open(path)?);
        let mut dictionaries = Vec::new();
        let dictionaries_count = data_file.read_u64::<BE>()?;

        for _ in 0..dictionaries_count {
            //Reading dictionary path
            let path_buffer_size = data_file.read_u64::<BE>()?;
            let mut path_buffer = vec![0u8; path_buffer_size as usize];
            data_file.read_exact(path_buffer.as_mut_slice())?;

            let dictionary_path = String::from_utf8(path_buffer.to_vec()).unwrap();
            let files_count = data_file.read_u64::<BE>()?; //Reading count of files in dictionary
            let mut files = Vec::with_capacity(files_count as usize);

            for _ in 0..files_count {
                //reading file name
                let file_name_size = data_file.read_u64::<BE>()?;
                let mut file_name_buffer = vec![0u8; file_name_size as usize];
                data_file.read_exact(file_name_buffer.as_mut_slice())?;
                let file_name = String::from_utf8(file_name_buffer).unwrap();

                let file_start = data_file.read_u64::<BE>()?;
                let file_end = data_file.read_u64::<BE>()?;
                data_file.set_position((file_end - file_start) + data_file.position());
                files.push(File::new(file_name, file_start, file_end));
            }
            dictionaries.push(Directory::new(dictionary_path, files));
        }
        Ok(Self::new::<P>(dictionaries, data_file.into_inner()))
    }
}
