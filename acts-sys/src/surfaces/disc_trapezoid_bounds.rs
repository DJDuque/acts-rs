pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/DiscTrapezoidBounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type DiscTrapezoidBounds;
        type DiscBounds = crate::surfaces::disc_bounds::DiscBounds;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_disc_trapezoid_bounds(
            half_x_inner_radius: f64,
            half_x_outer_radius: f64,
            inner_radius: f64,
            outer_radius: f64,
            average_phi: f64,
            stereo_angle: f64,
        ) -> UniquePtr<DiscTrapezoidBounds>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_disc_trapezoid_bounds(
            bounds: UniquePtr<DiscTrapezoidBounds>,
        ) -> UniquePtr<DiscBounds>;
    }
}
