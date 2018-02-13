fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => {
            let mut n = n - 2;
            let mut last = (1, 1);

            while n > 0 {
                last = (last.1, last.0 + last.1);
                n = n - 1;
            }

            last.1
        }
    }
}

fn main() {
    for n in 1..11 {
        println!("Fibonacci {} is: {}", n, fibonacci(n));
    }
}
