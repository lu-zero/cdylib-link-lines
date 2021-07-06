use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Manifest {
    pub package: Package,
    pub lib: Option<OptPackage>,
}

#[derive(Deserialize)]
pub struct OptPackage {
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
}

fn get_name() -> String {
    let mut s = String::new();
    let manifest_dir: PathBuf = env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    let path = manifest_dir.join("Cargo.toml");

    File::open(path).unwrap().read_to_string(&mut s).unwrap();

    let manifest: Manifest = toml::from_str(&s).unwrap();

    manifest
        .lib
        .and_then(|it| it.name)
        .unwrap_or(manifest.package.name)
}

pub fn metabuild() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    // We do not care about `_pre` and such.
    let major = env::var("CARGO_PKG_VERSION_MAJOR").unwrap();
    let minor = env::var("CARGO_PKG_VERSION_MINOR").unwrap();
    let patch = env::var("CARGO_PKG_VERSION_PATCH").unwrap();

    // Give the priority to [`cargo-c`](https://github.com/lu-zero/cargo-c) in case of.
    let prefix = PathBuf::from(env::var_os("CARGO_C_PREFIX").unwrap_or("/usr/local".into()));
    let libdir = env::var_os("CARGO_C_LIBDIR").map_or(prefix.join("lib"), Into::into);

    let target_dir = env::var_os("CARGO_TARGET_DIR").map_or(
        {
            let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
            manifest_dir
                .join("target")
                .join(std::env::var("PROFILE").unwrap())
        },
        Into::into,
    );

    let name = get_name();

    let lines = shared_object_link_args(
        &name, &major, &minor, &patch, &arch, &os, &env, libdir, target_dir,
    );

    for line in lines {
        println!("cargo:rustc-cdylib-link-arg={}", line);
    }
}

/// Return a list of linker arguments useful to produce a
/// platform-correct dynamic library.
pub fn shared_object_link_args(
    name: &str,
    major: &str,
    minor: &str,
    patch: &str,
    _arch: &str,
    os: &str,
    env: &str,
    libdir: PathBuf,
    target_dir: PathBuf,
) -> Vec<String> {
    let mut lines = Vec::new();

    match (os, env) {
        ("android", _) => {
            lines.push(format!("-Wl,-soname,lib{}.so", name));
        }

        ("linux", _) | ("freebsd", _) | ("dragonfly", _) | ("netbsd", _) if env != "musl" => {
            lines.push(format!("-Wl,-soname,lib{}.so.{}", name, major));
        }

        ("macos", _) | ("ios", _) => {
            lines.push(format!(
                "-Wl,-install_name,{1}/lib{0}.{2}.{3}.{4}.dylib,-current_version,{2}.{3}.{4},-compatibility_version,{2}",
                name,
                libdir.display(),
                major,
                minor,
                patch,
            ));
        }

        ("windows", "gnu") => {
            // This is only set up to work on GNU toolchain versions of Rust
            lines.push(format!(
                "-Wl,--out-implib,{}",
                target_dir.join(format!("{}.dll.a", name)).display()
            ));
            lines.push(format!(
                "-Wl,--output-def,{}",
                target_dir.join(format!("{}.def", name)).display()
            ));
        }

        _ => {}
    }

    lines
}
