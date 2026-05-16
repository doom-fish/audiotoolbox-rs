fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    for framework in ["AudioToolbox", "AudioUnit", "CoreAudio", "CoreFoundation"] {
        println!("cargo:rustc-link-lib=framework={framework}");
    }
}
