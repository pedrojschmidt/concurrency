use std::thread;
use std::time::Duration;

fn hello() {
    thread::spawn(|| println!("Hello from thread 1"));
    thread::spawn(|| println!("Hello from thread 2"));
}

fn hello_secuential()  {
    let t1 = thread::spawn(|| println!("Hello from thread 1"));
    let t2 = thread::spawn(|| println!("Hello from thread 2"));

    // Wait for both threads to complete
    t1.join().expect("t1 failed");
    t2.join().expect("t2 failed");
}

fn hello_scope()  {
    thread::scope(|s| {
        s.spawn(|| println!("Hello from thread 1"));
        s.spawn(|| println!("Hello from thread 2"));
    });
}

fn hello_scope_2()  {
    let n = 10;
    let mut m = 10;
    thread::scope(|s| {
        s.spawn(|| {
            m = m + 1;
            println!("Hello from thread 1, n = {n}")
        });
        s.spawn(|| println!("Hello from thread 2, n = {n}"));
    });
    println!("{m}");
}

fn main() {
    println!("====== Hello Threads    =======");
    hello();
    println!("====== Good Bye Threads =======");

    // uncomment me! thread::sleep(Duration::from_secs(1));
}
