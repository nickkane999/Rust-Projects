#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Russian,
    Japanese
}


pub fn run() {
    let language = Language::English;

    match language {
        Language::English => println!("Hello"),
        Language::Spanish => println!("Hola"),
        Language::Russian => println!("Privet"),
        Language::Japanese => println!("Konnichiwa"),
        lang => println!("Unknown language: {:?}", lang)
    }
}

pub fn run2() {
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Status: {}", status);
    } else if is_admin {
        println!("Admin");
    } else if let Ok(id) = group_id {
        println!("Group ID: {}", id);
    } else {
        println!("No authorization");
    }
}

pub fn run3() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(x) = stack.pop() {
        println!("{}", x);
    }
}

pub fn run4() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

pub fn run5() {
    let (x, y, z) = (1, 2, 3);

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
}

pub fn run6() {
    let point = (3, 5);

    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

pub fn run7() {
    // Irrefutable
    let x = 5;

    // Refutable
    let x: Option<i32> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // Can only accept irrefutable patterns
    // function parameters
    // let statements
    // for loops
}