pub use ffi::*;

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/MaterialDesignatorBlueprintNode.hpp");
        include!("acts-sys/include/geometry/cuboid_volume_bounds.hpp");
        include!("acts-sys/include/geometry/cylinder_volume_bounds.hpp");
        include!("acts-sys/include/helpers.hpp");

        type MaterialDesignatorBlueprintNode;
        type BlueprintNode = crate::geometry::blueprint_node::BlueprintNode;

        #[namespace = "acts_sys::ffi"]
        type CuboidVolumeBoundsFace = crate::geometry::cuboid_volume_bounds::CuboidVolumeBoundsFace;
        #[namespace = "acts_sys::ffi"]
        type CylinderVolumeBoundsFace =
            crate::geometry::cylinder_volume_bounds::CylinderVolumeBoundsFace;
        #[namespace = "Acts"]
        type DirectedProtoAxis = crate::utilities::proto_axis::DirectedProtoAxis;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_material_designator_blueprint_node(
            name: &CxxString,
        ) -> UniquePtr<MaterialDesignatorBlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_material_designator_blueprint_node(
            node: UniquePtr<MaterialDesignatorBlueprintNode>,
        ) -> UniquePtr<BlueprintNode>;

        #[rust_name = "configure_cylinder_face"]
        fn configureFace(
            self: Pin<&mut MaterialDesignatorBlueprintNode>,
            face: CylinderVolumeBoundsFace,
            loc0: &DirectedProtoAxis,
            loc1: &DirectedProtoAxis,
        ) -> Result<Pin<&mut MaterialDesignatorBlueprintNode>>;

        #[rust_name = "configure_cuboid_face"]
        fn configureFace(
            self: Pin<&mut MaterialDesignatorBlueprintNode>,
            face: CuboidVolumeBoundsFace,
            loc0: &DirectedProtoAxis,
            loc1: &DirectedProtoAxis,
        ) -> Result<Pin<&mut MaterialDesignatorBlueprintNode>>;
    }
}
