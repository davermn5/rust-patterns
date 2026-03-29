pub fn get_type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

//UNIT TESTS
#[cfg(test)]
mod helpers {
    use super::*;

    #[test]
    fn get_type_of_i32() {
        assert_eq!(get_type_of(&7), "i32");
    }
}