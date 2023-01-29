pub fn run() {
    let mut v1 = vec![1, 2, 3, 4, 5];

    for i in &mut v1 {
        *i += 50
    }

    for i in &v1 {
        println!("{}", i);
    }    

}