enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

pub fn run() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}