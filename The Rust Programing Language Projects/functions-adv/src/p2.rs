fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice<T>(f: T, arg: i32) -> i32 
where T: Fn(i32) -> i32 {
    f(arg) + f(arg)
}

//Fn, FnMut, and FnOnce


pub fn run() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}
///////////////////////////////////////////////

pub fn run2() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();

    println!("{:?}", list_of_strings);
}

///////////////////////////////////////////////

pub fn run3() {
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20)
        .map(Status::Value)
        .collect();
}

///////////////////////////////////////////////

fn returns_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |x| x + 1)
    } else {
        Box::new(move |x| x - 1)
    }
}

pub fn run4() {
    returns_closure(1);
}