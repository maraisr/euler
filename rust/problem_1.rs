fn main() {

    let mut count = 0u32;
    let mut result = 0u32;

    loop {
        count += 1;

        if count == 1000 {
            break;
        }

        if count % 3 == 0 || count % 5 == 0 {
            result = result + count;
        }
    }

    println!("{}", result);
    
}