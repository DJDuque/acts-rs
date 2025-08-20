pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/RadialBounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type RadialBounds;
        type DiscBounds = crate::surfaces::disc_bounds::DiscBounds;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_radial_bounds(
            inner_radius: f64,
            outer_radius: f64,
            half_phi: f64,
            average_phi: f64,
        ) -> UniquePtr<RadialBounds>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_radial_bounds(node: UniquePtr<RadialBounds>) -> UniquePtr<DiscBounds>;
    }
}
