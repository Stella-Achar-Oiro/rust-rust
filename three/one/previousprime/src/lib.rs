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

pub fn prev_prime(mut nbr: u64) -> u64 {
    // Handle edge cases
    if nbr <= 2 {
        return 0
    }
    
    // Start checking from nbr-1
    nbr -= 1;
    
    loop {
        if is_prime(nbr) {
            return nbr;
        }
        // Check if we've gone too low
        if nbr <= 2 {
            return 2; // 2 is the smallest prime
        }
        nbr -= 1;
    }
}