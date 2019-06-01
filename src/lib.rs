use std::env;
use std::path::PathBuf;

pub fn metabuild() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    // We do not care about `_pre` and such.
    let major = env::var("CARGO_PKG_VERSION_MAJOR").unwrap();
    let minor = env::var("CARGO_PKG_VERSION_MINOR").unwrap();
    let micro = env::var("CARGO_PKG_VERSION_PATCH").unwrap();

    let prefix: PathBuf = env::var_os("CARGO_C_PREFIX")
        .unwrap_or("/usr/local".into())
        .into();
    let libdir = env::var_os("CARGO_C_LIBDIR").map_or(prefix.join("lib"), |v| v.into());

    let target_dir = env::var_os("CARGO_TARGET_DIR").map_or(
        {
            let manifest_dir: PathBuf = env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
            manifest_dir
                .join("target")
                .join(std::env::var("PROFILE").unwrap())
        },
        |v| v.into(),
    );

    shared_object_link_line(&arch, &os, &env, &major, &minor, &micro, libdir, target_dir);
}

fn shared_object_link_line(
    _arch: &str,
    os: &str,
    env: &str,
    major: &str,
    minor: &str,
    micro: &str,
    libdir: PathBuf,
    target_dir: PathBuf,
) {
    let link = "cargo:rustc-cdylib-link-arg=";

    if os == "linux" {
        println!("{}-Wl,-soname,librav1e.so.{}", link, major);
    } else if os == "macos" {
        println!("{0}-Wl,-install_name,{1}/librav1e.{2}.{3}.{4}.dylib,-current_version,{2}.{3}.{4},-compatibility_version,{2}",
                link, libdir.display(), major, minor, micro);
    } else if os == "windows" && env == "gnu" {
        // This is only set up to work on GNU toolchain versions of Rust
        println!(
            "{}-Wl,--out-implib,{}",
            link,
            target_dir.join("rav1e.dll.a").display()
        );
        println!(
            "{}-Wl,--output-def,{}",
            link,
            target_dir.join("rav1e.def").display()
        );
    }
}
