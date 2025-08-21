pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/RectangleBounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type RectangleBounds;
        type PlanarBounds = crate::surfaces::planar_bounds::PlanarBounds;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_rectangle_bounds(half_x: f64, half_y: f64) -> UniquePtr<RectangleBounds>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_rectangle_bounds(
            bounds: UniquePtr<RectangleBounds>,
        ) -> UniquePtr<PlanarBounds>;
    }
}
