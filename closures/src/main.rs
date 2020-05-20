use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, I, O>
where
    T: Fn(I) -> O,
    I: Hash + Eq + Copy,
    O: Copy,
{
    calculation: T,
    values: HashMap<I, O>,
}

impl<T, I, O> Cacher<T, I, O>
where
    T: Fn(I) -> O,
    I: Hash + Eq + Copy,
    O: Copy,
{
    fn new(calculation: T) -> Cacher<T, I, O> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: I) -> O {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|intensity: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));

        intensity
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 30;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
