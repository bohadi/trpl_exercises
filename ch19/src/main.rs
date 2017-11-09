use std::slice;

static mut COUNTER: u32 = 0;

//unsafe allows:
//  deref raw pointer
//  access or modify mutable static var
//  call unsafe fn or method (or extern)
//  impl unsafe trait

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

fn add_to_count(x: u32) {
    unsafe {
        COUNTER += x;
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("just called rust from c");
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn main() {
    add_to_count(5);
    unsafe {
        println!("COUNTER: {}", COUNTER);
        println!("abs val of -3 according to c: {}", abs(-3));
    }
}
