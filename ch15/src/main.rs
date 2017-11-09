use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use List::{Cons, Nil};

fn _borrow(data: &i32) {
    println!("{}", data);
}
fn _mutable_borrow(data: &mut i32) {
    *data += 1;
    println!("{}", data);
}
fn _cause_panic(data: &RefCell<i32>) {
    //compiler wont complain -> runtime error
    let _a = data.borrow_mut();
    let _b = data.borrow_mut();
}
fn _f(data: &RefCell<i32>) {
    _borrow(&data.borrow());
    _mutable_borrow(&mut data.borrow_mut());
}

fn main() {
    let v1 = Rc::new(RefCell::new(1));
    let v2 = Rc::new(RefCell::new(2));
    let v3 = Rc::new(RefCell::new(3));
    
    let a = Cons(v1.clone(), Rc::new(Nil));
    let shared = Rc::new(a);

    let b = Cons(v2, shared.clone());
    let c = Cons(v3, shared.clone());

    *v1.borrow_mut() += 10;

    println!("{:?}", shared);
    println!("{:?}", b);
    println!("{:?}", c);
}
