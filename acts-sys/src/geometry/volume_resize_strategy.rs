pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    #[derive(Debug)]
    #[repr(i32)]
    enum VolumeResizeStrategy {
        Expand,
        Gap,
    }

    unsafe extern "C++" {
        include!("Acts/Geometry/VolumeResizeStrategy.hpp");

        type VolumeResizeStrategy;
    }
}
