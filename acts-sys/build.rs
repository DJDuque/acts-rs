fn main() {
    let dst = cmake::Config::new("acts")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_CXX_STANDARD", "20")
        .define("CMAKE_CXX_STANDARD_REQUIRED", "ON")
        .define("ACTS_USE_SYSTEM_EIGEN3", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=ActsCore");
}
