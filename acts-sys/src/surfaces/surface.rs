pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Definitions/Algebra.hpp");
        include!("Acts/Surfaces/ConeBounds.hpp");
        include!("Acts/Surfaces/ConeSurface.hpp");
        include!("Acts/Surfaces/CylinderBounds.hpp");
        include!("Acts/Surfaces/CylinderSurface.hpp");
        include!("Acts/Surfaces/DiscBounds.hpp");
        include!("Acts/Surfaces/DiscSurface.hpp");
        include!("Acts/Surfaces/Surface.hpp");
        include!("Acts/Surfaces/PlanarBounds.hpp");
        include!("Acts/Surfaces/PlaneSurface.hpp");
        include!("acts-sys/include/surfaces/surface.hpp");

        type Surface;

        type ConeBounds = crate::surfaces::cone_bounds::ConeBounds;
        type ConeSurface = crate::surfaces::cone_surface::ConeSurface;
        type CylinderBounds = crate::surfaces::cylinder_bounds::CylinderBounds;
        type CylinderSurface = crate::surfaces::cylinder_surface::CylinderSurface;
        type DiscBounds = crate::surfaces::disc_bounds::DiscBounds;
        type DiscSurface = crate::surfaces::disc_surface::DiscSurface;
        type PlanarBounds = crate::surfaces::planar_bounds::PlanarBounds;
        type PlaneSurface = crate::surfaces::plane_surface::PlaneSurface;
        type Transform3 = crate::definitions::algebra::Transform3;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "surface_factory"]
        fn make_shared_cone_surface(
            transform: &Transform3,
            bounds: SharedPtr<ConeBounds>,
        ) -> SharedPtr<ConeSurface>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "surface_factory"]
        fn make_shared_cylinder_surface(
            transform: &Transform3,
            bounds: SharedPtr<CylinderBounds>,
        ) -> SharedPtr<CylinderSurface>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "surface_factory"]
        fn make_shared_disc_surface(
            transform: &Transform3,
            bounds: SharedPtr<DiscBounds>,
        ) -> SharedPtr<DiscSurface>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "surface_factory"]
        fn make_shared_plane_surface(
            transform: &Transform3,
            bounds: SharedPtr<PlanarBounds>,
        ) -> SharedPtr<PlaneSurface>;
    }

    impl SharedPtr<Surface> {}
}
