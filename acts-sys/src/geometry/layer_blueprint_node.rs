use crate::surfaces::surface::Surface;
use cxx::{ExternType, SharedPtr};

pub use ffi::*;

// See https://github.com/dtolnay/cxx/issues/774 for why this is necessary.
// If the situation improves upstream, change this.
#[repr(transparent)]
pub struct SharedSurface {
    pub ptr: SharedPtr<Surface>,
}

unsafe impl ExternType for SharedSurface {
    type Id = cxx::type_id!("acts_sys::ffi::SharedSurface");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    #[namespace = "acts_sys::ffi"]
    #[derive(Debug)]
    #[repr(i32)]
    enum LayerBlueprintNodeLayerType {
        Cylinder,
        Disc,
        Plane,
    }

    unsafe extern "C++" {
        include!("Acts/Definitions/Algebra.hpp");
        include!("Acts/Geometry/LayerBlueprintNode.hpp");
        include!("acts-sys/include/geometry/layer_blueprint_node.hpp");
        include!("acts-sys/include/helpers.hpp");

        type LayerBlueprintNode;
        type StaticBlueprintNode = crate::geometry::static_blueprint_node::StaticBlueprintNode;

        #[namespace = "acts_sys::ffi"]
        type LayerBlueprintNodeLayerType;

        #[namespace = "Acts"]
        type ExtentEnvelope = crate::geometry::extent::ExtentEnvelope;
        #[namespace = "Acts"]
        type Transform3 = crate::definitions::algebra::Transform3;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_layer_blueprint_node(name: &CxxString) -> UniquePtr<LayerBlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_layer_blueprint_node(
            node: UniquePtr<LayerBlueprintNode>,
        ) -> UniquePtr<StaticBlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        fn layer_blueprint_node_set_surfaces(
            node: Pin<&mut LayerBlueprintNode>,
            surfaces: &CxxVector<SharedSurface>,
        );

        #[rust_name = "set_transform"]
        fn setTransform(
            self: Pin<&mut LayerBlueprintNode>,
            transform: &Transform3,
        ) -> Pin<&mut LayerBlueprintNode>;

        #[rust_name = "set_layer_type"]
        fn setLayerType(
            self: Pin<&mut LayerBlueprintNode>,
            layer_type: LayerBlueprintNodeLayerType,
        ) -> Pin<&mut LayerBlueprintNode>;

        #[rust_name = "set_use_center_of_gravity"]
        fn setUseCenterOfGravity(
            self: Pin<&mut LayerBlueprintNode>,
            x: bool,
            y: bool,
            z: bool,
        ) -> Pin<&mut LayerBlueprintNode>;

        #[rust_name = "set_extent_envelope"]
        fn setEnvelope(
            self: Pin<&mut LayerBlueprintNode>,
            envelope: &ExtentEnvelope,
        ) -> Pin<&mut LayerBlueprintNode>;
    }

    extern "C++" {
        include!("acts-sys/include/geometry/layer_blueprint_node.hpp");

        #[namespace = "acts_sys::ffi"]
        type SharedSurface = crate::geometry::layer_blueprint_node::SharedSurface;
    }
}
