fn main() {
    println!("{}", lpf(600851475143));
}

fn lpf(mut x: u64) -> u64 {

    let mut largest_prime = 2;

    while x > largest_prime {
        if x % largest_prime == 0 {
            x = x / largest_prime;
            largest_prime = 2;
        } else {
            largest_prime += 1;
        }
    }

    largest_prime
}