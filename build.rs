use pkg_config::probe_library;

fn main() {
    match pkg_config::probe_library("libnotify") {
        Ok(lib) => {
            // Link external library
            println!("cargo:rustc-link-lib=notify");
        },
        Err(e) => {
            println!("Error: {:?}", e);
            panic!();
        }
    }
}
