pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    //format!("Hello!") // Use if you want to show the error assert msg
}

#[cfg(test)]
mod tests_p3 {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }    

}