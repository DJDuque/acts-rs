pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/ConeSurface.hpp");

        type ConeSurface;
    }

    impl SharedPtr<ConeSurface> {}
}
