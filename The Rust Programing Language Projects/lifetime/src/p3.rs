use std::result;

pub fn run() {
    let str1 = String::from("abcd");
    let result;
    let str2;
    {
        // let str2 = String::from("xyz"); // Would error if str2 was not declared in above section
        str2 = String::from("xyz");
        result = longest(str1.as_str(), str2.as_str());
    }

    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}