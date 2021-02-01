use fizzbuzz::fizzbuzz;

fn main() {
    for n in 1..101 {
        println!("fizzbuzz({}): {}", n, fizzbuzz(n));
    }
}
