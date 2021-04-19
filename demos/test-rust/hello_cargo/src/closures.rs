use std::thread;
use std::time::Duration;

#[allow(dead_code)]
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    #[allow(dead_code)]
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    // 这个种写法是有问题的。当 arg 变化的时候，并不能缓存值
    #[allow(dead_code)]
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("do {} pushups", expensive_result.value(intensity));
        println!("do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
#[allow(dead_code)]
pub fn t_closures_int() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;

    assert!(equal_to_x(y));
}

#[cfg(test)]
mod closures_tests {
    use super::*;
    #[test]
    fn read_args_test() {
        t_closures_int();
    }
    #[test]
    #[should_panic]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
