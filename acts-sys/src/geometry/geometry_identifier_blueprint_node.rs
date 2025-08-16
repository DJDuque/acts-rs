pub use ffi::*;

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/GeometryIdentifierBlueprintNode.hpp");
        include!("acts-sys/include/helpers.hpp");

        type GeometryIdentifierBlueprintNode;
        type BlueprintNode = crate::geometry::blueprint_node::BlueprintNode;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new() -> UniquePtr<GeometryIdentifierBlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_mut(node: Pin<&mut GeometryIdentifierBlueprintNode>) -> Pin<&mut BlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique(
            node: UniquePtr<GeometryIdentifierBlueprintNode>,
        ) -> UniquePtr<BlueprintNode>;

        #[rust_name = "set_layer_id_to"]
        fn setLayerIdTo(
            self: Pin<&mut GeometryIdentifierBlueprintNode>,
            id: u64,
        ) -> Result<Pin<&mut GeometryIdentifierBlueprintNode>>;

        #[rust_name = "increment_layer_ids"]
        fn incrementLayerIds(
            self: Pin<&mut GeometryIdentifierBlueprintNode>,
            start: u64,
        ) -> Result<Pin<&mut GeometryIdentifierBlueprintNode>>;

        #[rust_name = "set_all_volume_ids_to"]
        fn setAllVolumeIdsTo(
            self: Pin<&mut GeometryIdentifierBlueprintNode>,
            id: u64,
        ) -> Result<Pin<&mut GeometryIdentifierBlueprintNode>>;
    }
}
