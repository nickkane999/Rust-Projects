mod p1;
mod p2;
mod p3;
mod p4;
fn main() {
    p1::run();
    p2::run();
    let p3_result = p3::run();
    match p3_result {
        Ok(value) => println!("The p3 result value is: {}", value),
        Err(error) => println!("The p3 result error is: {}", error),
    }
    p4::run();
}
