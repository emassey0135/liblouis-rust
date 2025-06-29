extern crate autotools;
extern crate bindgen;
extern crate log;
extern crate pkg_config;

use log::info;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = bindgen::Builder::default().header("wrapper.h");

    match pkg_config::Config::new()
        .atleast_version("3.9.0")
        .probe("liblouis")
    {
        Ok(system_liblouis) => {
            info!(
                "Found recent system liblouis via pkg-config. Version: {}",
                system_liblouis.version
            );
        }
        Err(e) => {
            info!("pkg-config error while trying to detect liblouis: {}", e);
            info!("building liblouis 3.34.0 from source");

            let dest = autotools::Config::new("liblouis-3.34.0")
                .enable("ucs4", None)
                .disable("dependency-tracking", None)
                .without("yaml", None)
                .enable_static()
                .disable_shared()
                .config_option("host", Some(&env::var("TARGET").unwrap()))
                .build();

            builder = builder.clang_args(["-I", &dest.join("include").to_str().unwrap()]);
            println!("cargo:rustc-link-search=native={}", dest.join("lib").display());
            println!("cargo:rustc-link-lib=static=louis");
        }
    };

    let bindings = builder.generate().unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
