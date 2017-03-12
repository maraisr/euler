use std::collections::HashMap;

fn main() {
    let mut mem: HashMap<i32, i32> = HashMap::new();

    let mut result = 0i32;
    let mut count = 0i32;

    loop {
        count = count + 1;

        let gen_fib = fib_gen(count, &mut mem);

        if gen_fib % 2 == 0 {
            result += gen_fib;
        }

        if gen_fib > 4000000 {
            break;
        }
    }

    println!("{}", result);
}

fn fib_gen(num: i32, mem: &mut HashMap<i32, i32>) -> i32 {
    if num <= 1 {
        return 1;
    }

    if let Some(val) = mem.get(&num) {
        return *val;
    }

    let num_a = fib_gen(num - 1, mem);
    let num_b = fib_gen(num - 2, mem);
    mem.insert(num - 1, num_a);
    mem.insert(num - 2, num_b);

    return num_a + num_b;
}