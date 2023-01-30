pub fn add_two(a: i32) -> i32 {
    a + 2
}
mod test_p6 {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }    

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_ne!(4, add_two(3));
    }    
}