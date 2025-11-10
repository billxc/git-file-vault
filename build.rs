use std::env;
use std::process::Command;

fn main() {
    // Check if this is a dev build
    let is_dev_build = env::var("DEV_BUILD").unwrap_or_default() == "true";
    
    let version = if is_dev_build {
        // Dev build: use timestamp version from environment (set by GitHub Actions)
        env::var("BUILD_VERSION").unwrap_or_else(|_| {
            // For local dev builds, use Cargo version + "-dev"
            get_local_dev_version()
        })
    } else {
        // Release build: use version from Cargo.toml
        env::var("CARGO_PKG_VERSION").unwrap()
    };
    
    // Set version environment variable for use in code
    println!("cargo:rustc-env=GFV_VERSION={}", version);
    
    // Also set git commit hash
    if let Some(commit_hash) = get_git_commit_hash() {
        println!("cargo:rustc-env=GFV_COMMIT_HASH={}", commit_hash);
    }
    
    // Rerun build script when these change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=DEV_BUILD");
    println!("cargo:rerun-if-env-changed=BUILD_VERSION");
}

fn get_local_dev_version() -> String {
    // Local dev build: append "-dev" to Cargo version
    let cargo_version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string());
    format!("{}-dev", cargo_version)
}

fn get_git_commit_hash() -> Option<String> {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
}
