use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn thread_spawning() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Spawned threads are shut down when the main thread completes.
    // Calling 'join' on the handle blocks the thread currently running
    // until the thread represented by the handle terminates.
    handle.join().unwrap();
}

pub fn move_closures() {
    let v = vec![1, 2, 3];
    // Closure may outlive the current function, but it borrows 'v'
    // 'move' forces the closure to take ownership of 'v'
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

pub fn channels() {
    // mpsc stands for 'multiple producer, single consumer'
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        // println!("val is: {}", vals) // Value used after being moved
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // recv blocks the main thread's execution and waits until a value is sent down the channel.
    // try_recv does not block, but instead returns a Result<T, E> immediately:
    // an Ok value holding the message if one is available
    // and an Err value if there aren't any messages this time.
    // A loop that calls try_recv every so often is needed.

    //let received = rx.recv().unwrap();

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn mutexes() {
    // Arc<T> - Atomic Reference Counting
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // The call to lock would fail if another thread holding the lock panicked.
            // In that case, on one would be able to get the lock, so we've chosen to unwrap
            // and have this thread panic if we're in that situation.
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}