pub use ffi::*;

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/ContainerBlueprintNode.hpp");
        include!("acts-sys/include/helpers.hpp");

        type CylinderContainerBlueprintNode;
        type CuboidContainerBlueprintNode;
        type ContainerBlueprintNode;

        #[namespace = "Acts"]
        type AxisDirection = crate::utilities::axis_definitions::AxisDirection;
        #[namespace = "Acts"]
        type VolumeAttachmentStrategy =
            crate::geometry::volume_attachment_strategy::VolumeAttachmentStrategy;
        #[namespace = "Acts"]
        type VolumeResizeStrategy = crate::geometry::volume_resize_strategy::VolumeResizeStrategy;
        type BlueprintNode = crate::geometry::blueprint_node::BlueprintNode;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_cylinder_container_blueprint_node(
            name: &CxxString,
            axis: AxisDirection,
            attachment_strategy: VolumeAttachmentStrategy,
            resize_strategy: VolumeResizeStrategy,
        ) -> UniquePtr<CylinderContainerBlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_cylinder_container_blueprint_node(
            node: UniquePtr<CylinderContainerBlueprintNode>,
        ) -> UniquePtr<ContainerBlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_cuboid_container_blueprint_node(
            name: &CxxString,
            axis: AxisDirection,
            attachment_strategy: VolumeAttachmentStrategy,
            resize_strategy: VolumeResizeStrategy,
        ) -> UniquePtr<CuboidContainerBlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_cuboid_container_blueprint_node(
            node: UniquePtr<CuboidContainerBlueprintNode>,
        ) -> UniquePtr<ContainerBlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique_container_blueprint_node(
            node: UniquePtr<ContainerBlueprintNode>,
        ) -> UniquePtr<BlueprintNode>;
    }
}
