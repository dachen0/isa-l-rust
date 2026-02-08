fn main() {
    let mut cfg = cmake::Config::new("isa-l");

    cfg.define("BUILD_SHARED_LIBS", "OFF")
        .define("ISAL_BUILD_TESTS", "OFF")
        .define("BUILD_FUZZ_TESTS", "OFF")
        .define("BUILD_ISAL_SHIM", "OFF");

    let dst = cfg.build();

    // Link the static library
    println!(
        "cargo:rustc-link-search=native={}/lib",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=isal");

    // Re-run if isa-l source changes
    println!("cargo:rerun-if-changed=isa-l/");
}
