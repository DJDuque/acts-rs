pub use ffi::*;

#[cxx::bridge(namespace = "Acts")]
mod ffi {
    unsafe extern "C++" {
        include!("Acts/Definitions/Direction.hpp");
        include!("acts-sys/include/definitions/direction.hpp");

        type Direction;

        #[namespace = "acts_sys::ffi"]
        fn new_negative_direction() -> UniquePtr<Direction>;

        #[namespace = "acts_sys::ffi"]
        fn new_positive_direction() -> UniquePtr<Direction>;

        fn sign(&self) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_negative_direction() {
        let direction = new_negative_direction();
        assert_eq!(direction.sign(), -1);
    }

    #[test]
    fn test_new_positive_direction() {
        let direction = new_positive_direction();
        assert_eq!(direction.sign(), 1);
    }
}
