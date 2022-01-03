// FIXME: rust-android-gradle にて platforms 固有の library のサポートされていないので記載している
const NDK_VERSION: &str = "22.1.7171670";
const API_LEVEL: &str = "29";

pub fn ndk() -> String {
    std::env::var("ANDROID_SDK_ROOT").expect("ANDROID_NDK variable not set")
}

struct Target {
    architecture: String,
    vendor: String,
    system: String,
    abi: Option<String>,
    platforms_lib_path: String,
}

fn main() {
    let target_str = std::env::var("TARGET").unwrap();
    let target: Vec<String> = target_str.split('-').map(|s| s.into()).collect();
    if target.len() < 3 {
        assert!(!(target.len() < 3), "Failed to parse TARGET {}", target_str);
    }

    let architecture = target[0].clone();

    let abi = if target.len() > 3 {
        Some(target[3].clone())
    } else {
        None
    };

    let arch = if architecture == "aarch64" {
        "arm64".into()
    } else {
        architecture.clone()
    };

    let platforms_lib_path = format!(
        "{}/ndk/{}/platforms/android-{}/arch-{}/usr/lib",
        &ndk(),
        &NDK_VERSION,
        &API_LEVEL,
        &arch
    );

    let target = Target {
        architecture,
        vendor: target[1].clone(),
        system: target[2].clone(),
        abi,
        platforms_lib_path,
    };

    println!("cargo:rerun-if-changed=build.rs");
    println!("system {:?}", &target.system);

    // for var in std::env::vars() {
    //     println!("system {}={}", var.0, var.1);
    // }

    println!(
        "cargo:rustc-link-search=native={}",
        &target.platforms_lib_path
    );
    println!("cargo:rustc-link-lib=jnigraphics");
}
