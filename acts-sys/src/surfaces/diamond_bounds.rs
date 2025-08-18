pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/DiamondBounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type DiamondBounds;
        type PlanarBounds = crate::surfaces::planar_bounds::PlanarBounds;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_diamond_bounds(
            half_x_neg_y: f64,
            half_x_zero_y: f64,
            half_x_pos_y: f64,
            half_y_neg: f64,
            half_y_pos: f64,
        ) -> UniquePtr<DiamondBounds>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_diamond_bounds(node: UniquePtr<DiamondBounds>) -> UniquePtr<PlanarBounds>;
    }
}
