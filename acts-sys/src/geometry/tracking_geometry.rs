pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/TrackingGeometry.hpp");

        type TrackingGeometry;

        type GeometryContext = crate::geometry::geometry_context::GeometryContext;
        type IVisualization3D = crate::visualization::i_visualization3d::IVisualization3D;
        type ViewConfig = crate::visualization::view_config::ViewConfig;

        fn visualize(
            self: &TrackingGeometry,
            helper: Pin<&mut IVisualization3D>,
            geometry_context: &GeometryContext,
            global_config: &ViewConfig,
            portal_config: &ViewConfig,
            sensitive_config: &ViewConfig,
        );
    }

    impl UniquePtr<TrackingGeometry> {}
}
