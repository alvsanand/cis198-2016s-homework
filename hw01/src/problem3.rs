/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = vec![];

    for i in 2..n {
        let mut is_prime = true;

        for j in 0..primes.len() {
            if i as u32 % primes[j] as u32 == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(i)
        }
    }

    primes
}
