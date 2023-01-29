pub fn run() {
    let r = 3;

    // Commenting out the following line will cause the compiler to complain due to lifetime issues.
    /*
    {
        let x = 5;
        r = &x;
    }
    */

    println!("r: {}", r);
}