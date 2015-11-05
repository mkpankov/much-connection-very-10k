use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::net;
use std::io::Read;

fn main() {
    let listener = Arc::new(Mutex::new(net::TcpListener::bind("127.0.0.1:10042").unwrap()));
    let counter = Arc::new((Mutex::new(0), Condvar::new()));
    let mut threads: Vec<_> = Vec::with_capacity(10000);
    for _ in 0..10000 {
        let listener = listener.clone();
        let counter = counter.clone();
        let t = thread::spawn(move || {
            let (mut stream, _) = listener.lock().unwrap().accept().unwrap();
            let mut string = String::with_capacity(8);
            stream.read_to_string(&mut string).unwrap();
            println!("{}", string);

            let &(ref lock, ref condvar) = &*counter;
            let mut value = lock.lock().unwrap();
            *value += 1;
            if *value == 10000 {
                condvar.notify_one();
            }
        });
        threads.push(t);
    }
    let &(ref lock, ref condvar) = &*counter;
    let mut value = lock.lock().unwrap();
    while *value != 10000 {
        value = condvar.wait(value).unwrap();
    }
    let n: usize = *value;
    println!("{:?}", n);
}
