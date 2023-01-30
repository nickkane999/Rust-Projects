fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod tests_p2 {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }    

    #[test]
    fn it_does_not_add_two() {
        assert_ne!(4, add_two(3));
    }    
}