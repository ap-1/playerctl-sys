use std::env;
use std::path::PathBuf;

fn main() {
    // Build playerctl using meson.
    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("build");
    let playerctl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("playerctl");

    // Link to the system glib-2.0 shared library.
    pkg_config::Config::new()
        .atleast_version("2.72")
        .probe("glib-2.0")
        .unwrap();

    // Link to the playerctl shared library.
    match pkg_config::Config::new()
        .atleast_version("2.4.1")
        .probe("playerctl")
    {
        Ok(_) => (),
        Err(_) => {
            meson::build(
                playerctl_path.to_str().unwrap(),
                build_path.to_str().unwrap(),
            );

            // Link to the built playerctl shared library.
            println!("cargo:rustc-link-lib=playerctl");
            println!(
                "cargo:rustc-link-search=native={}",
                build_path.join("playerctl").display()
            );
        }
    }

    // Invalidate the built crate whenever the input header changes.
    let input_header = playerctl_path.join("playerctl").join("playerctl.h");
    println!("cargo:rerun-if-changed={}", input_header.display());

    let bindings = bindgen::Builder::default()
        // Set the input header to generate bindings for.
        .header(input_header.to_str().unwrap())
        // Include glib-object.h and glibconfig.h, respectively.
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib/glib-2.0/include")
        // Allow relevant playerctl definitions to be generated.
        .allowlist_var("Playerctl_.*")
        .allowlist_type("Playerctl_.*")
        .allowlist_function("playerctl_.*")
        // Block extraneous glib definitions from being generated.
        .blocklist_type(r"(?i)\b_?G\w+")
        // Invalidate the built crate whenever any header file changes.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
