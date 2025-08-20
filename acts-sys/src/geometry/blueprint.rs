pub use ffi::*;

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/Blueprint.hpp");
        include!("acts-sys/include/geometry/blueprint.hpp");
        include!("acts-sys/include/helpers.hpp");

        type Blueprint;
        type BlueprintOptions = crate::geometry::blueprint_options::BlueprintOptions;

        #[namespace = "acts_sys::ffi"]
        type BlueprintConfig;

        #[namespace = "Acts"]
        type ExtentEnvelope = crate::geometry::extent::ExtentEnvelope;
        #[namespace = "Acts"]
        type GeometryContext = crate::geometry::geometry_context::GeometryContext;
        #[namespace = "Acts"]
        type Logger = crate::utilities::logger::Logger;
        #[namespace = "Acts"]
        type TrackingGeometry = crate::geometry::tracking_geometry::TrackingGeometry;

        #[namespace = "acts_sys::ffi"]
        fn new_blueprint_config(envelope: &ExtentEnvelope) -> UniquePtr<BlueprintConfig>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_blueprint(config: &BlueprintConfig) -> UniquePtr<Blueprint>;

        fn construct(
            self: Pin<&mut Blueprint>,
            options: &BlueprintOptions,
            context: &GeometryContext,
            logger: &Logger,
        ) -> UniquePtr<TrackingGeometry>;
    }
}
