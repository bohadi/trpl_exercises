use std::fmt::Display;

fn find_largest<T>(list: &[T]) -> &T
    where T: PartialOrd + Display, 
{
    let mut largest = &list[0];
    for i in list.iter() {
        if i > largest { largest = &i; }
    }
    largest
}
fn run_ex1() {
    let l = vec![10., 1., 4., 2., 10.1, 0.9];
    let ll = find_largest(&l);
    println!("{}", ll.to_string());
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 } 
}
fn run_ex2() {
    let s1 = String::from("abc");
    let s2 = "abcde";
    println!("{}", longest(s1.as_str(), s2));

}

fn main() {
    run_ex1();
    run_ex2();
}