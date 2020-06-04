use std::env;
use std::fs;
use std::path;
use std::process;

fn main() {
    let out = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    let cmake_build_dir = out.join("build");

    // The cmark library is checked out as a submodule in a directory
    // called 'cmark' at the root of this repo.
    let cmake_src_dir = path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("cmark");

    if !path::Path::new(&cmake_build_dir).exists() {
        fs::create_dir(&cmake_build_dir).unwrap();
    }

    // Run CMake's configure step so it generates a Makefile.
    process::Command::new("cmake")
        .current_dir(&cmake_build_dir)
        .arg(cmake_src_dir.clone())
        .arg("-DBUILD_SHARED_LIBS=OFF")
        .arg(format!(
            "-DCMAKE_INSTALL_PREFIX={}",
            cmake_build_dir.display()
        ))
        .status()
        .unwrap();

    // Tell CMake to build the library and install it somewhere
    // we know where to find it so we can link to it later.
    process::Command::new("cmake")
        .current_dir(&cmake_build_dir)
        .arg("--build")
        .arg(".")
        .arg("--target")
        .arg("install")
        .status()
        .unwrap();

    // Tell the compiler where we put the built libcmark.a.
    println!("cargo:rustc-flags=-L{}", cmake_build_dir.display());
    println!("cargo:rustc-link-lib=cmark");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell the compiler where to find the cmark header
        // files that we installed in a previous step above.
        .clang_arg(format!("-I{}/include/", cmake_build_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // FIXME: Without this override, bindgen adds some malformed
        // #[doc] stuff to the bindgen.rs which does not compile.
        .generate_comments(false)
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("couldn't write bindings");
}
