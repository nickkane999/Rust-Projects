pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = largest(char_list);
    println!("The largest number is {}", largest);
}

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}