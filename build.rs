use std::env;
use std::path::PathBuf;

fn main() {
    // Link the system playerctl shared library from the specified directory.
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=playerctl");
    println!("cargo:rustc-link-lib=glib-2.0");

    // Invalidate the built crate whenever the wrapper changes.
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        // Set the input header to generate bindings for.
        .header("wrapper.h")
        // Include glib-object.h and glibconfig.h, respectively.
        .clang_arg("-I/usr/include/glib-2.0")
        .clang_arg("-I/usr/lib/glib-2.0/include")
        // Allow relevant playerctl definitions to be generated.
        .allowlist_var("Playerctl_.*")
        .allowlist_type("Playerctl_.*")
        .allowlist_function("playerctl_.*")
        // Allow relevant glib definitions to be generated.
        .allowlist_function("g_list_.*")
        .allowlist_function("g_error_.*")
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
