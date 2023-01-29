pub fn run() {
    a();
 

fn a() {
    b();
}

fn b() {
    c(21);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Crash and burn");
    }
}}