pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        println!("{} не є простим числом", n);
        return false;
    }

    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            println!("{} не є простим числом", n);
            return false;
        }
    }

    println!("{} є простим числом", n);
    true
}