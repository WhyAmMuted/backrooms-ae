use std::{
    env,
    path::{Path, PathBuf},
};

#[inline(always)]
fn get_platform_dir(name: &str, sdk_root: &Path) -> PathBuf {
    match name {
        "x86_64-unknown-linux-gnu" => sdk_root.join("lib/linux-x64"),
        "windows-x64" => sdk_root.join("lib/windows-x64"),
        _ => {
            panic!("Unsupported platform!");
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let sdk_root = manifest_dir.join("steamaudio");
    let sdk_root = sdk_root
        .canonicalize()
        .expect("Directory of 'steamaudio' module not found");

    let header_path = sdk_root.join("include/phonon.h");
    let header_path_str = header_path.to_str().unwrap();

    let platform = std::env::var("TARGET").unwrap();
    println!("{:?}", platform);
    let lib_path = get_platform_dir(PathBuf::from(platform).to_str().unwrap(), &sdk_root);

    println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=phonon");
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,{}",
        lib_path.to_str().unwrap()
    );

    let bindings = bindgen::Builder::default()
        .header(header_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Can't generate bindgen.rs");
    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Can't write in bindings.rs");
}
