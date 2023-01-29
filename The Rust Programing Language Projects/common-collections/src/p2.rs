pub fn run() {
    let v1 = vec![1, 2, 3, 4, 5];

    //let third: &i32 = &v1[2];
    //println!("The third element is {}", third);

    match v1.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}