use std::env;
use std::path::PathBuf;

fn main() {
    // Allow override
    if let Ok(dir) = env::var("CJSON_DIR") {
        let p = PathBuf::from(dir);
        println!("cargo:rustc-link-search=native={}", p.display());
        println!("cargo:rustc-link-lib=dylib=cjson");
        println!("cargo:rustc-link-lib=dylib=cjson_utils");
        return;
    }

    // Prefer local workspace build if present
    let workspace_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let candidate = PathBuf::from(&workspace_manifest_dir)
        .join("..")
        .join("build-host")
        .join("cJSON")
        .join("build");

    if candidate.exists() {
        println!("cargo:rustc-link-search=native={}", candidate.display());
        // prefer dynamic linking if available
        println!("cargo:rustc-link-lib=dylib=cjson");
        println!("cargo:rustc-link-lib=dylib=cjson_utils");
        return;
    }

    // Try pkg-config for libcjson_utils and libcjson (system-wide)
    let mut found_pkg = false;
    if pkg_config::Config::new().probe("libcjson_utils").is_ok() {
        found_pkg = true;
    }
    if pkg_config::Config::new().probe("libcjson").is_ok() {
        found_pkg = true;
    }
    if found_pkg {
        // pkg-config will emit metadata automatically
        return;
    }

    // (other fallbacks tried above)

    // If we reach here we couldn't find cJSON; emit a helpful message but allow build to continue
    println!("cargo:warning=Could not find cJSON via CJSON_DIR, pkg-config, or ../build-host/cJSON/build. Tests requiring cJSON may fail.");
}
