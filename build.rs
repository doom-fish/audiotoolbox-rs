use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-env-changed=DEVELOPER_DIR");
    println!("cargo:rerun-if-env-changed=SDKROOT");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=AudioUnit");
    println!("cargo:rustc-link-lib=framework=AVFAudio");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-arg=-mmacosx-version-min=13.0");

    let swift_dir = "swift-bridge";
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR should be set by Cargo");
    let swift_build_dir = format!("{out_dir}/swift-build");

    println!("cargo:rerun-if-changed={swift_dir}");

    if let Ok(output) = Command::new("swiftlint")
        .args(["lint"])
        .current_dir(swift_dir)
        .output()
    {
        if !output.status.success() {
            eprintln!(
                "SwiftLint warnings:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        }
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let swift_triple = match target_arch.as_str() {
        "x86_64" => "x86_64-apple-macosx",
        "aarch64" => "arm64-apple-macosx",
        other => {
            panic!("audiotoolbox: unsupported target arch '{other}'. Expected x86_64 or aarch64.")
        }
    };

    let output = Command::new("swift")
        .args([
            "build",
            "-c",
            "release",
            "--triple",
            swift_triple,
            "--package-path",
            swift_dir,
            "--scratch-path",
            &swift_build_dir,
        ])
        .output()
        .expect("Failed to build Swift bridge");

    if !output.status.success() {
        eprintln!(
            "Swift build STDOUT:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        eprintln!(
            "Swift build STDERR:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!(
            "Swift build failed with exit code: {:?}",
            output.status.code()
        );
    }

    println!("cargo:rustc-link-search=native={swift_build_dir}/release");
    println!("cargo:rustc-link-lib=static=AudioToolboxBridge");
    println!("cargo:rustc-link-lib=framework=Foundation");

    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    println!("cargo:rustc-link-search=native=/usr/lib/swift");

    match Command::new("xcode-select").arg("-p").output() {
        Ok(output) if output.status.success() => {
            let xcode_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let legacy_path = format!(
                "{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift-5.5/macosx"
            );
            println!("cargo:rustc-link-arg=-Wl,-rpath,{legacy_path}");
            println!("cargo:rustc-link-search=native={legacy_path}");
            let modern_path =
                format!("{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{modern_path}");
            println!("cargo:rustc-link-search=native={modern_path}");
            println!("cargo:rustc-link-lib=static=swiftCompatibility56");
            println!("cargo:rustc-link-lib=static=swiftCompatibilityConcurrency");
            println!("cargo:rustc-link-lib=static=swiftCompatibilityPacks");
        }
        Ok(output) => {
            println!(
                "cargo:warning=`xcode-select -p` exited non-zero (status={:?}); Swift runtime rpaths were not added.",
                output.status.code()
            );
        }
        Err(err) => {
            println!(
                "cargo:warning=`xcode-select` could not be invoked ({err}); Swift runtime rpaths were not added."
            );
        }
    }
}
