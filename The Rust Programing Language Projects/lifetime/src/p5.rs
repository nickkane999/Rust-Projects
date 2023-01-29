use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn run() {
    let str1 = String::from("abcd");
    let str2 = String::from("xyzzzzzzz");

    let result = longest_with_an_announcement(str1.as_str(), str2.as_str(), "Hello");
    println!("The longest string is {}", result);
}