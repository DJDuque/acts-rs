fn main() {
    let dst = cmake::Config::new("acts")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_CXX_STANDARD", "20")
        .define("CMAKE_CXX_STANDARD_REQUIRED", "ON")
        .define("ACTS_USE_SYSTEM_EIGEN3", "OFF")
        .build();

    let bridge_files = vec![
        "src/definitions/algebra.rs",
        "src/definitions/alignment.rs",
        "src/definitions/common.rs",
        "src/definitions/direction.rs",
        "src/definitions/pdg_particle.rs",
        "src/definitions/particle_data.rs",
        "src/definitions/tolerance.rs",
        "src/definitions/track_parametrization.rs",
        "src/definitions/units.rs",
        "src/geometry/blueprint_node.rs",
        "src/geometry/container_blueprint_node.rs",
        "src/geometry/cuboid_volume_bounds.rs",
        "src/geometry/cylinder_volume_bounds.rs",
        "src/geometry/geometry_identifier_blueprint_node.rs",
        "src/geometry/material_designator_blueprint_node.rs",
        "src/geometry/static_blueprint_node.rs",
        "src/geometry/volume_attachment_strategy.rs",
        "src/geometry/volume_resize_strategy.rs",
        "src/surfaces/diamond_bounds.rs",
        "src/surfaces/planar_bounds.rs",
        "src/surfaces/rectangle_bounds.rs",
        "src/surfaces/trapezoid_bounds.rs",
        "src/utilities/axis_definitions.rs",
        "src/utilities/proto_axis.rs",
    ];
    cxx_build::bridges(bridge_files)
        .include("./include")
        .include(dst.join("include"))
        .include(dst.join("include").join("eigen3"))
        .std("c++20")
        .compile("acts-sys");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=ActsCore");
}
