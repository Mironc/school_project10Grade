mod ext;
mod loader;
#[allow(dead_code)]
pub mod saver;
pub mod model_importer;
pub mod image_importer;
//pub mod ranged_reader;
pub use loader::*;
const COMPRESS_LEVEL: i32 = 12;
/// Data file structure is next:
/// count of directories
/// Directory {
///     Path(size+data)
///     Count of files
///     Array of files
/// }
/// File {
///     Name(size+data)
///     start byte
///     end byte
///     file data
/// }
#[cfg(test)]
mod test {
    use crate::*;
    use std::{fs::*, path::Path};
    fn read_file(path: impl AsRef<Path>) -> String {
        read_to_string(path).unwrap()
    }
    #[test]
    pub fn test() {
        let timer = std::time::Instant::now();
        saver::create_data_file("testmaterials").unwrap();
        println!(
            "creating data file taken {} seconds",
            timer.elapsed().as_secs_f32()
        );
        let timer = std::time::Instant::now();
        let mut assets = loader::Assets::from_data_file("assets.data").unwrap();
        dbg!(&assets);
        let file = assets
            .open_file_string("Pushkin's poetry/Curious.txt")
            .unwrap();
        assert_eq!(
            read_file("testmaterials/Pushkin's poetry/Curious.txt"),
            file
        );
        let file = assets
            .open_file_string("Pushkin's poetry/Friendship.txt")
            .unwrap();
        assert_eq!(
            read_file("testmaterials/Pushkin's poetry/Friendship.txt"),
            file
        );
        let file = assets
            .open_file_string("Pushkin's poetry/Merry feast.txt")
            .unwrap();
        assert_eq!(
            read_file("testmaterials/Pushkin's poetry/Merry feast.txt"),
            file
        );
        assert_eq!(
            read_file("testmaterials/stanford-bunny.obj"),
            assets.open_file_string("stanford-bunny.obj").unwrap()
        );
        println!(
            "reading data file taken {} seconds",
            timer.elapsed().as_secs_f32()
        );
    }
}
