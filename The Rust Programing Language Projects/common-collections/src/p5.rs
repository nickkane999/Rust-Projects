pub fn run() {
     let s1 = String::new();
     let s2 = "initial contents";
     let s3 = s2.to_string();
     let s4 = String::from("initial contents");

     let mut s = String::from("foo");
        s.push_str("bar");
        s.push('!');
        
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    //let s7 = s5 + &s6; // note s5 has been moved here and can no longer be used
    let s7 = format!("{}{}", s5, s6);

    let hello = String::from("Hello");
    let c = &hello[0];
}