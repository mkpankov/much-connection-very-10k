use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::net;
use std::io::{Read, Write};

fn main() {
    const THREADS_NUM: usize = 10000;

    let listener = Arc::new(Mutex::new(net::TcpListener::bind("127.0.0.1:10042").unwrap()));
    let counter = Arc::new((Mutex::new(0), Condvar::new()));
    let mut threads: Vec<_> = Vec::with_capacity(THREADS_NUM);
    for i in 0..THREADS_NUM {
        let listener = listener.clone();
        let counter = counter.clone();
        let t = thread::spawn(move || {
            let (mut stream, _) = listener.lock().unwrap().accept().unwrap();
            let mut buf = Vec::new();
            stream.read(&mut buf).unwrap();
            let response = b"OK";
            stream.write_all(response).unwrap();

            stream.shutdown(std::net::Shutdown::Both).unwrap();

            let &(ref lock, ref condvar) = &*counter;
            let mut value = lock.lock().unwrap();
            println!("{}: {}", i, *value);
            *value += 1;
            if *value == THREADS_NUM {
                condvar.notify_one();
            }
        });
        threads.push(t);
    }
    let &(ref lock, ref condvar) = &*counter;
    let mut value = lock.lock().unwrap();
    while *value != THREADS_NUM {
        value = condvar.wait(value).unwrap();
    }
    let n: usize = *value;
    println!("{:?}", n);
}
