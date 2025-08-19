pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/LineSurface.hpp");

        type LineSurface;
    }

    impl SharedPtr<LineSurface> {}
}
