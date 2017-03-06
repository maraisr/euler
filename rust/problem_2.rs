fn main() {
    let mut window: [i32; 2] = [1, 1];
    let mut result = 0i32;

    loop {
        let tmp_a = window[0];
        let tmp_b = window[1];

        window[0] = window[1];
        window[1] = tmp_a + tmp_b;

        if window[1] % 2 == 0 {
            result += window[1];
        }

        if window[1] > 4000000 {
            break;
        }
    }

    println!("{}", result);
}