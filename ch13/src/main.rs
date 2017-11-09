use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Counter {
    pub count: u8,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.count += 1;
        if self.count < 6
            { Some( self.count ) }
        else
            { None }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    pub size: u8,
    pub style: String,
}
fn iterators() {
    let v1  = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    let v2: Vec<_> = v2.iter().map(|x| x-1).collect();
    assert_eq!(v1, v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 12,
            style: String::from("boot"),
        },
        Shoe {
            size: 8,
            style: String::from("sandal"),
        }
    ];

    let s1: Vec<Shoe> = shoes.into_iter().filter(|s| s.size > 10).collect();
    assert_eq!(
        s1,
        vec![ Shoe { size: 12, style: String::from("boot") } ]
    );

    let c: u8 = Counter::new()
        .skip(1)
        .filter(|x| x % 2 == 1)
        .map(|x| x*2)
        .sum();
    println!("{}", c);
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    fn value(&mut self, x: u32) -> &u32 {
        if !self.values.contains_key(&x) {
            self.values.insert(x, (self.calculation)(x) );
        }
        self.values.get(&x).unwrap()
    }
}
fn generate_workout(intensity: u32) {
    let closure = |x| {
        thread::sleep(Duration::from_secs(1));
        x
    };
    let mut cacher = Cacher::new(closure);

    if intensity < 5 {
        println!("take a break");
        if intensity == 3 { println!("{} min run", cacher.value(intensity)); }
    } else {
        println!("{} pushups", cacher.value(intensity));
        println!("{} situps", cacher.value(2*intensity));
    }

}

fn main() {
    generate_workout(10);
    iterators();
}
