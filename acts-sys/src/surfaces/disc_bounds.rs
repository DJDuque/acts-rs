pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/DiscBounds.hpp");

        type DiscBounds;
    }

    impl UniquePtr<DiscBounds> {}
}
