use std::{env::var, fs, path::{Path, PathBuf}};

fn main() {
    #[cfg(target_os = "windows")] {
        println!("cargo:rustc-link-search=static=lib");
        println!("cargo:rustc-link-lib=static=mpv");

        let target_dir = get_output_path();
        let lib_dir = Path::new(&var("CARGO_MANIFEST_DIR").unwrap()).join("lib\\mpv-1.dll");
        let dest = Path::new(&target_dir).join("mpv-1.dll");
        fs::copy(lib_dir, dest).unwrap();

        fn get_output_path() -> PathBuf {
            let manifest_dir_string = var("CARGO_MANIFEST_DIR").unwrap();
            let build_type = var("PROFILE").unwrap();
            let path = Path::new(&manifest_dir_string).join("target").join(build_type);
            return PathBuf::from(path);
        }
    }
}