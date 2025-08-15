pub use ffi::*;

#[cxx::bridge(namespace = "Acts::Experimental")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Geometry/BlueprintNode.hpp");

        type BlueprintNode;

        #[rust_name = "add_child"]
        fn addChild(
            self: Pin<&mut Self>,
            child: SharedPtr<BlueprintNode>,
        ) -> Result<Pin<&mut BlueprintNode>>;
    }
}
