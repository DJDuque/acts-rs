pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Definitions/Algebra.hpp");
        include!("Acts/Surfaces/Surface.hpp");
        include!("Acts/Surfaces/PlanarBounds.hpp");
        include!("Acts/Surfaces/PlaneSurface.hpp");
        include!("acts-sys/include/surfaces/surface.hpp");

        type Surface;

        type PlanarBounds = crate::surfaces::planar_bounds::PlanarBounds;
        type PlaneSurface = crate::surfaces::plane_surface::PlaneSurface;
        type Transform3 = crate::definitions::algebra::Transform3;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "surface_factory"]
        fn make_shared_plane_surface(
            transform: &Transform3,
            bounds: SharedPtr<PlanarBounds>,
        ) -> SharedPtr<PlaneSurface>;
    }
}
