use std::thread;
use std::time::Duration;

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

#[derive(Copy, Clone)]
struct Cacher<T, U>
    where T: Fn(U) -> U,
        U: Copy,
        T: Copy
{
    calculation: T,
    value: Option<U>,
}

impl<T, U> Cacher<T, U>
    where T: Fn(U) -> U,
    U: Copy,
    T: Copy
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: U) -> U {
        match &self.value {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}