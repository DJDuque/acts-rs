pub use ffi::*;

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/LayerBlueprintNode.hpp");
        include!("acts-sys/include/helpers.hpp");

        type LayerBlueprintNode;
        type StaticBlueprintNode = crate::geometry::static_blueprint_node::StaticBlueprintNode;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_layer_blueprint_node(name: &CxxString) -> UniquePtr<LayerBlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_layer_blueprint_node(
            node: UniquePtr<LayerBlueprintNode>,
        ) -> UniquePtr<StaticBlueprintNode>;
    }
}
