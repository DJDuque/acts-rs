pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/ConeBounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type ConeBounds;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_cone_bounds(
            opening_angle: f64,
            min_z: f64,
            max_z: f64,
            half_phi: f64,
            average_phi: f64,
        ) -> UniquePtr<ConeBounds>;
    }

    impl SharedPtr<ConeBounds> {}
}
