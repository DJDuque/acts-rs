pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/TrapezoidBounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type TrapezoidBounds;
        type PlanarBounds = crate::surfaces::planar_bounds::PlanarBounds;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_trapezoid_bounds(
            half_x_neg_y: f64,
            half_x_pos_y: f64,
            half_y: f64,
            rot_angle: f64,
        ) -> UniquePtr<TrapezoidBounds>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_trapezoid_bounds(
            bounds: UniquePtr<TrapezoidBounds>,
        ) -> UniquePtr<PlanarBounds>;
    }
}
