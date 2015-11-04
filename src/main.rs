use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::net;
use std::io::Read;

fn main() {
    let listener = Arc::new(Mutex::new(net::TcpListener::bind("127.0.0.1:10042").unwrap()));
    let counter = Arc::new(AtomicUsize::new(0));
    let mut threads: Vec<_> = Vec::with_capacity(10000);
    for _ in 0..10000 {
        let listener = listener.clone();
        let counter = counter.clone();
        let t = thread::spawn(move || {
            let (mut stream, _) = listener.lock().unwrap().accept().unwrap();
            let mut string = String::with_capacity(8);
            stream.read_to_string(&mut string).unwrap();
            println!("{}", string);

            counter.fetch_add(1, Ordering::Relaxed);
        });
        threads.push(t);
    }
    std::thread::sleep(std::time::Duration::new(10, 0));
    println!("{:?}", counter);
}
