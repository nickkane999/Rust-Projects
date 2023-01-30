pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    /*
    fn for_each<F>(self, f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        for x in self {
            f(x)
        }
    }
    */
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for (index, val) in v1_iter.enumerate() {
        assert_eq!(*val, v1[index]);
    }
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

pub fn run() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }
}