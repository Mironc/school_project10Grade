use std::{
    fs,
    io::{ copy, BufReader, BufWriter, Result, Seek, Write},
    path::{Path, PathBuf},
};
use zstd::stream::copy_encode;

use crate::{ext::PathEXT, COMPRESS_LEVEL};
use byteorder::{WriteBytesExt, BE};
#[derive(Debug, Clone)]
pub struct File {
    file_source: PathBuf,
    name: String,
}
impl File {
    pub fn new(name: &str, path: impl AsRef<Path>) -> Self {
        Self {
            name: name.to_owned(),
            file_source: path.as_ref().into(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Directory {
    path: String, 
    file_count: usize,
    files: Vec<File>,
}
fn create_file(path: impl AsRef<Path>) -> Result<fs::File> {
    fs::File::create(path)
}
fn open_file(path: impl AsRef<Path>) -> Result<fs::File> {
    fs::OpenOptions::new().read(true).open(path)
}
pub fn create_data_file_to(assets_folder: impl AsRef<Path>,data_file_folder_path:impl AsRef<Path>) -> Result<()>{
    let mut data_file = fs::File::create(data_file_folder_path)?;
    let mut data_file_writer = BufWriter::new(data_file.try_clone()?);

    let directories = get_directories(assets_folder.as_ref(), assets_folder.as_ref())?;
    data_file_writer.write_u64::<BE>(directories.len() as u64)?;
    for dir in directories.iter() {
        // TODO: maybe use hashes instead??
        data_file_writer.write_u64::<BE>(dir.path.chars().count() as u64)?;//writing directory size len
        data_file_writer.write_all(dir.path.as_bytes())?;//writing directory name
        data_file_writer.write_u64::<BE>(dir.file_count as u64)?;//writing file count
        for file in dir.files.iter() {
            data_file_writer.write_u64::<BE>(file.name.len() as u64)?;//writing file name len
            data_file_writer.write_all(file.name.as_bytes())?;//writing file name
            let offset = data_file_writer.stream_position()?+16;// 16 considers writing 2 u64
            data_file_writer.write_u64::<BE>(offset)?;
            let mut compressed_data = Vec::new();
            copy_file_compressed(&mut compressed_data, file)?;
            data_file_writer.write_u64::<BE>(offset + compressed_data.len() as u64)?;
            data_file_writer.write(compressed_data.as_slice())?;
        }
    }
    data_file.flush().unwrap();
    Ok(())
}
///creates asset file
pub fn create_data_file(assets_folder: impl AsRef<Path>) -> Result<()> {
    let data_file_path = &assets_folder.as_ref().parent().unwrap().join("assets.data");
    create_data_file_to(assets_folder, data_file_path)
}
///not in use
fn copy_file_to(mut destination: impl Write, file: &File) -> Result<()> {
    let mut opened_file = BufReader::new(std::fs::File::open(&file.file_source)?);
    let timer = std::time::Instant::now();
    println!("{} bytes copied", copy(&mut opened_file, &mut destination)?);
    println!("writing taken {} secs", timer.elapsed().as_secs_f32());
    Ok(())
}
fn copy_file_compressed(mut destination: &mut impl Write, file: &File) -> Result<()> {
    let mut opened_file = BufReader::new(std::fs::File::open(&file.file_source)?);
    let timer = std::time::Instant::now();
    copy_encode(&mut opened_file, &mut destination, COMPRESS_LEVEL)?;
    println!("writing taken {} secs", timer.elapsed().as_secs_f32());
    Ok(())
}
///recursevely handle every dictionary,creates vec of directories and files
fn get_directories(
    root_path: impl AsRef<Path> + std::marker::Copy,
    path: impl AsRef<Path>,
) -> Result<Vec<Directory>> {
    let mut value = Vec::new();
    let mut files = Vec::new();
    for entry in fs::read_dir(path.as_ref())? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            value.append(&mut get_directories(root_path, path)?);
        } else if path.is_file() {
            files.push(File::new(&path.get_name().unwrap(), path))
        }
    }
    value.push(Directory {
        file_count: files.len(),
        files,
        path: path
            .as_ref()
            .strip_prefix(root_path)
            .unwrap()
            .to_string()
            .unwrap(),
    });
    Ok(value)
}
