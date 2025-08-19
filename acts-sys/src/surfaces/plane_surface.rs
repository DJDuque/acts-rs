pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Surfaces/PlaneSurface.hpp");
        include!("Acts/Surfaces/RegularSurface.hpp");
        include!("acts-sys/include/helpers.hpp");

        type PlaneSurface;
        type RegularSurface = crate::surfaces::regular_surface::RegularSurface;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_shared_plane_surface(node: SharedPtr<PlaneSurface>) -> SharedPtr<RegularSurface>;
    }
}
