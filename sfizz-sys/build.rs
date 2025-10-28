use std::env;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(err) = try_main() {
        panic!("{}", err);
    }
}

fn try_main() -> anyhow::Result<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let workspace_root = manifest_dir.parent().expect("workspace root");
    let vendor_dir = workspace_root.join("vendor").join("sfizz");

    println!("cargo:rerun-if-env-changed=SFIZZ_SYS_LIB_DIR");
    println!("cargo:rerun-if-env-changed=SFIZZ_SYS_INC_DIR");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SYSTEM");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_VENDORED");
    println!("cargo:rerun-if-changed={}", vendor_dir.display());

    if env::var_os("CARGO_FEATURE_SYSTEM").is_some() {
        link_system_library()?;
    } else {
        build_vendored(&vendor_dir)?;
    }

    generate_bindings(&vendor_dir)?;

    Ok(())
}

fn link_system_library() -> anyhow::Result<()> {
    let lib = pkg_config::Config::new()
        .atleast_version("2")
        .probe("sfizz")?;
    for dir in lib.include_paths {
        println!("cargo:include={}", dir.display());
    }
    if let Ok(lib_dir) = env::var("SFIZZ_SYS_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    }
    Ok(())
}

fn build_vendored(vendor_dir: &Path) -> anyhow::Result<()> {
    if !vendor_dir.exists() {
        anyhow::bail!("sfizz repo missing at {}", vendor_dir.display());
    }

    let mut config = cmake::Config::new(vendor_dir);
    config.profile("Release");
    config.define("BUILD_TESTING", "OFF");
    config.define("SFIZZ_JACK", "OFF");
    config.define("SFIZZ_AUDIOFILES", "OFF");
    config.define("SFIZZ_SHARED", "OFF");
    config.define("SFIZZ_DOCS", "OFF");
    config.define("SFIZZ_RENDER", "OFF");

    let build = config.build();
    let lib_dir = build.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=sfizz");

    Ok(())
}

fn generate_bindings(vendor_dir: &Path) -> anyhow::Result<()> {
    let fallback_include = vendor_dir.join("src").join("sfizz");
    let header_root = env::var_os("SFIZZ_SYS_INC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| fallback_include.clone());
    let header = header_root.join("sfizz.h");
    if !header.exists() {
        anyhow::bail!(
            "sfizz C header not found. Set SFIZZ_SYS_INC_DIR or clone vendor sources. Tried {}",
            header.display()
        );
    }

    let mut builder = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .clang_arg(format!("-I{}", header_root.display()))
        .allowlist_function("sfizz_.*")
        .allowlist_type("sfizz_.*")
        .allowlist_var("SFIZZ_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    if fallback_include != header_root {
        builder = builder.clang_arg(format!("-I{}", fallback_include.display()));
    }

    let bindings = builder.generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR")?).join("sfizz_bindings.rs");
    bindings.write_to_file(&out_path)?;

    Ok(())
}
