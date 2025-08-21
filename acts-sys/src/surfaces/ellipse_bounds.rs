pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/EllipseBounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type EllipseBounds;
        type PlanarBounds = crate::surfaces::planar_bounds::PlanarBounds;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_ellipse_bounds(
            inner_radius_x: f64,
            inner_radius_y: f64,
            outer_radius_x: f64,
            outer_radius_y: f64,
            half_phi: f64,
            average_phi: f64,
        ) -> UniquePtr<EllipseBounds>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_ellipse_bounds(
            bounds: UniquePtr<EllipseBounds>,
        ) -> UniquePtr<PlanarBounds>;
    }
}
