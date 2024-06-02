use rustc_version::{version_meta, Channel};

fn main() {
    // Set cfg flags depending on release channel
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo::rustc-check-cfg=cfg(CHANNEL_NIGHTLY)");
        println!("cargo:rustc-cfg=CHANNEL_NIGHTLY");
    }
}
