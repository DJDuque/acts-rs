pub use ffi::*;

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/StaticBlueprintNode.hpp");
        include!("acts-sys/include/helpers.hpp");

        type StaticBlueprintNode;
        type BlueprintNode = crate::geometry::blueprint_node::BlueprintNode;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_static_blueprint_node(
            node: UniquePtr<StaticBlueprintNode>,
        ) -> UniquePtr<BlueprintNode>;
    }
}
