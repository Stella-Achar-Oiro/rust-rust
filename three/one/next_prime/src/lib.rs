pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let sqrt = (n as f64).sqrt() as u64;
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn next_prime(mut nbr: u64) -> u64 {
    loop {
        if is_prime(nbr) {
            return nbr;
        }
        nbr += 1;
    }
}

