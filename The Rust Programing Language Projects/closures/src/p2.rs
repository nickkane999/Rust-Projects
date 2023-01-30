pub fn run() {
    let x = 4;
    /*
    fn equal_to_x(x: i32, z: i32) -> bool { return z == x; }
    */
    let equal_to_x = |z| z == x;
    let y = 4;
    //let result = equal_to_x(x, y);
    //assert!(equal_to_x(x, y));
    assert!(equal_to_x(y));
}