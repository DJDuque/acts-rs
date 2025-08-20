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
        #[rust_name = "transform_equivalent_tolerance"]
        fn s_transformEquivalentTolerance() -> f64;

        #[namespace = "acts_sys::ffi"]
        fn make_transform3(
            rotation_x: f64,
            rotation_y: f64,
            rotation_z: f64,
            translation_x: f64,
            translation_y: f64,
            translation_z: f64,
        ) -> UniquePtr<Transform3>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_equivalent_tolerance() {
        let tol = transform_equivalent_tolerance();
        assert_eq!(tol, 1e-9);
    }
}
