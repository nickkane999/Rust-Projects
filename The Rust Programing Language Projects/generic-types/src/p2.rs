struct Point<T> {
    x: T,
    y: T,
}
/* 
struct Point<T, U> {
    x: T,
    y: U,
}

*/

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}


pub fn run() {
    let p1 = Point { x: 5, y: 10 };
    p1.x();
    let p2 = Point { x: 5.0, y: 10.0 };
    p2.y();
    //let p3 = Point { x: 5, y: 10.0 };

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
