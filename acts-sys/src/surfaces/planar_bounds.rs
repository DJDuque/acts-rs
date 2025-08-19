pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/PlanarBounds.hpp");

        type PlanarBounds;
    }

    impl UniquePtr<PlanarBounds> {}
    impl SharedPtr<PlanarBounds> {}
}
