pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/CylinderSurface.hpp");
        include!("Acts/Surfaces/RegularSurface.hpp");
        include!("acts-sys/include/helpers.hpp");

        type CylinderSurface;
        type RegularSurface = crate::surfaces::regular_surface::RegularSurface;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_shared_cylinder_surface(
            node: SharedPtr<CylinderSurface>,
        ) -> SharedPtr<RegularSurface>;
    }
}
