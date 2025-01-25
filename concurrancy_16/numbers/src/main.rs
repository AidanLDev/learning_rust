use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    /*
     // thread iteration example
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");

            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    */

    /*
    // Thread with vector example
    let v = vec![1, 2, 3,];

    let handle = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap()
    */

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");

        tx.send(val).unwrap();
    });
    let recieved = rx.recv().unwrap();

    println!("Got: {recieved}");
}
