pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/DiscSurface.hpp");
        include!("Acts/Surfaces/RegularSurface.hpp");
        include!("acts-sys/include/helpers.hpp");

        type DiscSurface;
        type RegularSurface = crate::surfaces::regular_surface::RegularSurface;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_shared_disc_surface(node: SharedPtr<DiscSurface>) -> SharedPtr<RegularSurface>;
    }
}
