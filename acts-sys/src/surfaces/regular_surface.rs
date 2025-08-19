pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/RegularSurface.hpp");
        include!("acts-sys/include/helpers.hpp");

        type RegularSurface;
        type Surface = crate::surfaces::surface::Surface;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_shared_regular_surface(node: SharedPtr<RegularSurface>) -> SharedPtr<Surface>;
    }
}
