pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/DiscSurface.hpp");

        type DiscSurface;
    }

    impl SharedPtr<DiscSurface> {}
}
