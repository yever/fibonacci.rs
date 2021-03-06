fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => (1..n)
                .fold((1, 1), |(fst, snd), _| (snd, fst + snd))
                .1
    }
}

fn main() {
    for n in 0..21 {
        println!("Fibonacci {} is: {}", n, fibonacci(n));
    }
}
