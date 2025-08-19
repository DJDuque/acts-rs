pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/Extent.hpp");
        include!("acts-sys/include/geometry/extent.hpp");

        type ExtentEnvelope;

        #[namespace = "acts_sys::ffi"]
        fn new_extent_envelope(
            x: &[f64; 2],
            y: &[f64; 2],
            z: &[f64; 2],
            r: &[f64; 2],
            phi: &[f64; 2],
            r_phi: &[f64; 2],
            theta: &[f64; 2],
            eta: &[f64; 2],
            mag: &[f64; 2],
        ) -> UniquePtr<ExtentEnvelope>;
    }
}
