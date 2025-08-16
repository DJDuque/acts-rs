pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    #[derive(Debug)]
    #[repr(i32)]
    enum VolumeAttachmentStrategy {
        First,
        Second,
        Midpoint,
        Gap,
    }

    unsafe extern "C++" {
        include!("Acts/Geometry/VolumeAttachmentStrategy.hpp");

        type VolumeAttachmentStrategy;
    }
}
