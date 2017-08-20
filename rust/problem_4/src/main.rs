use std::cmp;
use std::i32;

fn main() {

    let mut a: i32 = 100;
    let mut b: i32 = 100;
    let mut max_a = i32::min_value();
    let mut max_b = i32::min_value();

    while a <= 999 {
        while b <= 999 {
            if is_palindome(a * b) {
                max_a = cmp::max(max_a, a);
                max_b = cmp::max(max_b, b);
            }

            b += 1;
        }
        a += 1;
    }

    print!("{} x {} = {}", max_a, max_b, max_a * max_b);

}

fn is_palindome(x: i32) -> bool {

    let mut tracker = x;
    let n = x;
    let mut rev = 0;

    while tracker > 0 {
        let dig = tracker % 10;
        rev = rev * 10 + dig;
        tracker = tracker / 10;
    }

    n == rev
}