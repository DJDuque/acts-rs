pub use ffi::*;

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/ContainerBlueprintNode.hpp");
        // See https://github.com/acts-project/acts/pull/4556
        // Remove this include if/when the PR is merged
        include!("Acts/Geometry/PortalShell.hpp");
        include!("acts-sys/include/helpers.hpp");

        type ContainerBlueprintNode;
        type BlueprintNode = crate::geometry::blueprint_node::BlueprintNode;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_mut(node: Pin<&mut ContainerBlueprintNode>) -> Pin<&mut BlueprintNode>;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "upcast"]
        fn upcast_unique(node: UniquePtr<ContainerBlueprintNode>) -> UniquePtr<BlueprintNode>;
    }
}
