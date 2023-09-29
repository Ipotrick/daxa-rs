extern crate bindgen;

fn main() {
    let static_crt = std::env::var("CARGO_ENCODED_RUSTFLAGS")
        .unwrap_or_default()
        .contains("target-feature=+crt-static");
    let dst = cmake::Config::new("daxa")
        .build_target("daxa")
        .profile(get_profile())
        .configure_arg("-DBUILD_SHARED_LIBS=OFF")
        .configure_arg("-DDAXA_USE_VCPKG=ON")
        .configure_arg(format!(
            "-DDAXA_USE_STATIC_CRT={}",
            if static_crt { 1 } else { 0 }
        ))
        .build();
    println!(
        "cargo:rustc-link-search=native={}/build/{}",
        dst.display(),
        get_profile()
    );
    println!("cargo:rustc-link-lib=static=daxa");
    use std::env;
    use std::path::PathBuf;
    println!("cargo:rerun-if-changed=src/daxa.h");
    println!("cargo:rerun-if-changed=daxa");
    
    // TODO: x64-windows? Consideration must be made for other OSes and even cross-compile.
    let vcpkg_includes = format!(
        "-I{}/build/vcpkg_installed/x64-windows/include",
        dst.display()
    );
    let bindings = bindgen::Builder::default()
        .clang_arg("--target=x86_64-pc-windows-msvc")
        .clang_arg("--language=c")
        .clang_arg("-Idaxa/include")
        .clang_arg(vcpkg_includes)
        .header("src/daxa.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn get_profile() -> &'static str {
    // For now, just use release. Thanks Douglas!
    // https://github.com/rust-lang/rust/issues/39016
    "Release"
}
