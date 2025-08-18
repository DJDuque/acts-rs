pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/LineBounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type LineBounds;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_line_bounds(radius: f64, half_z: f64) -> UniquePtr<LineBounds>;
    }
}
