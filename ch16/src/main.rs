use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::time::Duration;

//Send indicates ownership may be transferred to another thread
//Sync indicates access from multiple threads is safe
//  T is Sync is &T is Send

fn channels() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    let _h1 = thread::spawn(move || {
        let vals = vec![
            String::from("1hi"),
            String::from("1from"),
            String::from("1the"),
            String::from("1thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });
    let _h2 = thread::spawn(move || {
        let vals = vec![
            String::from("2hello"),
            String::from("2this"),
            String::from("2is"),
            String::from("2another"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    for msg in rx {
        println!("got: {}", msg);
    }
}

fn mutexes() {
    let x = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let x = x.clone();
        let h = thread::spawn(move || {
            let mut val = x.lock().unwrap();
            *val += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("result: {}", *x.lock().unwrap());
}

fn main() {
    channels();
    mutexes();
}