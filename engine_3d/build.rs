use assets::saver::*;
use std::path::*;
pub fn main() {
    let pre_assets_dir = std::env::var("OUT_DIR").unwrap();
    let mut out_path = Path::new(&pre_assets_dir);
    for _ in 0..5 {
        out_path = out_path.parent().unwrap()
    }
    let assets_dir = out_path.join("./assets");
    println!("{}", assets_dir.display());
    if cfg!(debug_assertions) {
        create_data_file(assets_dir).unwrap();
    } else {
        let out_path = Path::new(&out_path).join("./target/release/assets.data");
        println!("{}",out_path.display());

        create_data_file_to(
            assets_dir,
            out_path,
        )
        .unwrap();
    }
}
