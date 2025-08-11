pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Definitions/Algebra.hpp");
        include!("acts-sys/include/definitions/algebra.hpp");

        type Vector2;
        type Vector3;
        type Vector4;
        type SquareMatrix2;
        type SquareMatrix3;
        type SquareMatrix4;
        type Translation2;
        type Translation3;
        type RotationMatrix2;
        type RotationMatrix3;
        type AngleAxis3;
        type Transform2;
        type Transform3;

        #[namespace = "acts_sys::ffi"]
        fn transform_equivalent_tolerance() -> f64;
    }
}
