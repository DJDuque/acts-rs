pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Utilities/ProtoAxis.hpp");
        include!("acts-sys/include/helpers.hpp");

        type DirectedProtoAxis;
        type AxisDirection = crate::utilities::axis_definitions::AxisDirection;
        type AxisBoundaryType = crate::utilities::axis_definitions::AxisBoundaryType;

        #[namespace = "acts_sys::ffi"]
        #[cxx_name = "make_unique"]
        fn new_directed_proto_axis(
            dir: AxisDirection,
            boundary_type: AxisBoundaryType,
            edges: &CxxVector<f64>,
        ) -> UniquePtr<DirectedProtoAxis>;
    }
}
