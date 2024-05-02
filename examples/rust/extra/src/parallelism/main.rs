use std::thread;

// 2 * k * a * t * exp(-a * t * t)
fn evaluate_sequential(k: f64, a: f64, t:f64) -> f64 {

    let a = 2.0 * k * a * t;
    let b = f64::exp(-a * t * t);

    return a * b;
}

fn evaluate_parallel(k: f64, a: f64, t:f64) -> f64 {

    let thread1 = thread::spawn(
        || 2.0 * k * a * t
    );
    let thread2 = thread::spawn(
        || f64::exp(-a * t * t)
    );

    // join returns the result of the expression
    let a = thread1.join().unwrap();
    let b = thread2.join().unwrap();

    return a * b;
}
