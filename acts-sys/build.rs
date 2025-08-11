fn main() {
    let dst = cmake::Config::new("acts")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_CXX_STANDARD", "20")
        .define("CMAKE_CXX_STANDARD_REQUIRED", "ON")
        .define("ACTS_USE_SYSTEM_EIGEN3", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=ActsCore");

    let bridge_files = vec![
        "src/definitions/algebra.rs",
        "src/definitions/alignment.rs",
        "src/definitions/common.rs",
        "src/definitions/direction.rs",
        "src/definitions/pdg_particle.rs",
    ];
    cxx_build::bridges(bridge_files)
        .include("./include")
        .include(dst.join("include"))
        .include(dst.join("include").join("eigen3"))
        .compile("acts-sys");
}
