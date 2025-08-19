pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/CylinderSurface.hpp");

        type CylinderSurface;
    }

    impl SharedPtr<CylinderSurface> {}
}
