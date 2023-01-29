struct ImportantException<'a> {
    part: &'a str,
}

impl<'a> ImportantException<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn run() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantException {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}