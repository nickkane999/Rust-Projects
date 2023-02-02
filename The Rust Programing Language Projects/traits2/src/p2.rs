struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Age(u32);
struct ID(u32);

pub fn run1 () {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

//////////////////////////////////////////////////////////////////////////////

pub fn run2 () {
    type Kilometers = i32;

    let x = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
} 

//////////////////////////////////////////////////////////////////////////////

pub fn run3 () {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hi"))
    }
}

//////////////////////////////////////////////////////////////////////////////
// Defines guess with only 1 value, otherwise gets undefined
pub fn run4 () {
    while game_in_progress() {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }    
}

//////////////////////////////////////////////////////////////////////////////

impl<T> Option<T> {
    pub fn unwrap(self, default: T) -> T {
        match self {
            Some(val) => val,
            None => panic!("Called `Option::unwrap()` on a `None` value"),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

pub fn run5 () {
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
}

fn generic<T: ?Sized>(t: &T) {
    // --snip--
}