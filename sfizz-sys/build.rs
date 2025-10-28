use std::env;
use std::fs;
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

    link_platform_libraries();
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
    config.define("SFIZZ_AUDIOFILES", "ON");
    config.define("SFIZZ_USE_SNDFILE", "ON");
    config.define("SFIZZ_SNDFILE_STATIC", "OFF");
    config.define("SFIZZ_SHARED", "OFF");
    config.define("SFIZZ_DOCS", "OFF");
    config.define("SFIZZ_RENDER", "OFF");
    config.define("CMAKE_CXX_STANDARD", "17");
    config.build_target("sfizz_static");

    let install_root = config.build();
    let install_lib = install_root.join("lib");
    if install_lib.exists() {
        println!("cargo:rustc-link-search=native={}", install_lib.display());
    }
    let build_lib = install_root.join("build").join("library").join("lib");
    if build_lib.exists() {
        println!("cargo:rustc-link-search=native={}", build_lib.display());
    }
    let mut static_libs = Vec::new();
    if let Ok(entries) = fs::read_dir(&build_lib) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("a") {
                if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                    if let Some(stripped) = file_name.strip_prefix("lib").and_then(|s| s.strip_suffix(".a")) {
                        static_libs.push(stripped.to_string());
                    }
                }
            }
        }
    }
    static_libs.sort();

    #[cfg(target_family = "unix")]
    println!("cargo:rustc-link-arg=-Wl,--start-group");

    for lib in &static_libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    #[cfg(target_family = "unix")]
    println!("cargo:rustc-link-arg=-Wl,--end-group");

    println!("cargo:rustc-link-lib=dylib=sndfile");

    Ok(())
}

fn generate_bindings(vendor_dir: &Path) -> anyhow::Result<()> {
    let fallback_include = vendor_dir.join("src");
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    if fallback_include != header_root {
        builder = builder.clang_arg(format!("-I{}", fallback_include.display()));
    }

    let bindings = builder.generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR")?).join("sfizz_bindings.rs");
    bindings.write_to_file(&out_path)?;

    Ok(())
}

fn link_platform_libraries() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=pthread");
        println!("cargo:rustc-link-lib=dylib=m");
    }

    #[cfg(target_os = "freebsd")]
    println!("cargo:rustc-link-lib=dylib=c++");

    #[cfg(all(
        target_family = "unix",
        not(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))
    ))]
    println!("cargo:rustc-link-lib=dylib=c++");
}
