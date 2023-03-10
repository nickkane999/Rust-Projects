

pub fn main2() {
    /*
    enum Option<T> {
        Some(T),
        None,
    }
    */

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("sum: {}", sum);
}

