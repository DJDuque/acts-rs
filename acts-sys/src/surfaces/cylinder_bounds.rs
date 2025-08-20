pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/CylinderBounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type CylinderBounds;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_cylinder_bounds(
            radius: f64,
            half_z: f64,
            half_phi: f64,
            average_phi: f64,
            bevel_neg_z: f64,
            bevel_pos_z: f64,
        ) -> UniquePtr<CylinderBounds>;
    }

    impl SharedPtr<CylinderBounds> {}
}
