mod hof;

fn main() {
    let r = vec![1, 10, 20];
    println!("{:?}", hof::map(r, |x| x * x));
}
