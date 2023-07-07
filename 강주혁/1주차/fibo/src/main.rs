fn main() {
    for num in 1..10 {
        println!("{}", fibo(num));
    }
}

fn fibo(n: u32) -> u32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 1;
    } else {
        return fibo(n - 1) + fibo(n - 2);
    }
}
