use std::convert::TryInto;
use std::vec::Vec;

use crate::euclid;

const PRIME_THRES: u64 = 10_000;

pub fn is_prime(x: i64) -> bool {
    let x: u64 = x.try_into().unwrap();

    if x < PRIME_THRES {
        is_prime_td(x)
    } else if cannot_be_prime_td(x) {
        false
    } else {
        is_prime_mr(x)
    }
}

fn is_prime_td(x: u64) -> bool {
    if x < 2 {
        return false;
    }
    if (2..=3).contains(&x) {
        return true;
    }

    if x & 1 == 0 {
        return false;
    }
    if x % 3 == 0 {
        return false;
    }

    let mut k = 6;

    while k - 1 <= x / (k - 1) {
        if x % (k - 1) == 0 {
            return false;
        }
        if x % (k + 1) == 0 {
            return false;
        }
        k += 6;
    }
    true
}

fn cannot_be_prime_td(x: u64) -> bool {
    // cannot evaluate small x
    if x < 100 {
        return false;
    }

    if x & 1 == 0 {
        return true;
    }
    if x % 3 == 0 {
        return true;
    }

    // k must test up to the largest base value of MR's prime test
    let mut k = 6;

    while k < 100 {
        if x % (k - 1) == 0 {
            return true;
        }
        if x % (k + 1) == 0 {
            return true;
        }
        k += 6;
    }
    false
}

fn is_prime_mr(x: u64) -> bool {
    let mut num = x - 1;
    let mut pow = 0;

    while num & 1 == 0 {
        pow += 1;
        num >>= 1;
    }
    let num_odd = num;

    let bases: [u64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];

    for base in bases.iter() {
        let mut q: u64 = euclid::mod_exp_u64(*base, num_odd, x);
        if q == 1 || q == x - 1 {
            continue;
        }
        let mut jump = false;

        for _i in 0..pow - 1 {
            q = euclid::mod_mult_u64(q, q, x);
            if q == x - 1 {
                jump = true;
                break;
            }
        }
        if jump {
            continue;
        }
        return false;
    }
    true
}

fn safe_square_for_positive_num(x: u64) -> u64 {
    if x < u64::MAX / x {
        x * x
    } else {
        0
    }
}

fn fermats_factorization(n: u64) -> (u64, u64) {
    if n <= 1 || n & 1 == 0 {
        return (0, 0);
    }
    let mut a: u64 = (n as f64).sqrt().ceil() as u64;

    let mut a_sq = safe_square_for_positive_num(a);
    if a_sq == 0 {
        return (0, 0);
    }
    if a_sq == n {
        return (a, a);
    }

    for _ in 1..5 {
        let b_sq = a_sq - n; // always > 0
        let b = (b_sq as f64).sqrt().floor() as u64;

        if safe_square_for_positive_num(b) == b_sq {
            return (a - b, a + b); // smaller first
        } else {
            a += 1;
            a_sq = safe_square_for_positive_num(a);

            if a_sq == 0 {
                return (0, 0);
            }
        }
    }
    (0, 0)
}

pub fn factorize(mut n: u64, factors: &mut Vec<u64>) {
    while n & 1u64 == 0 {
        n >>= 1;
        factors.push(2);
    }
    while n % 3 == 0 {
        n /= 3;
        factors.push(3);
    }
    while n % 5 == 0 {
        n /= 5;
        factors.push(5);
    }
    while n % 7 == 0 {
        n /= 7;
        factors.push(7);
    }

    let wheel_inc = [
        2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6, 4, 6, 2,
        4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10,
    ];

    let wheels = wheel_inc.len();
    let mut k: u64 = 11;
    let mut i: usize = 0;

    while k <= n / k {
        if n % k == 0 {
            n /= k;
            factors.push(k);
        } else {
            k += wheel_inc[i];
            if i < wheels - 1 {
                i += 1;
            } else {
                let ferm_res = fermats_factorization(n);
                if ferm_res.0 > 1 {
                    factors.push(ferm_res.0);
                    factors.push(ferm_res.1);
                    n = 1;
                    break;
                }
                i = 0;
            }
        }
    }
    if n != 1 {
        factors.push(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_td_for_primes() {
        assert!(is_prime_td(2) == true);
        assert!(is_prime_td(11) == true);
        assert!(is_prime_td(1009) == true);
        assert!(is_prime_td(8863) == true);
    }

    #[test]
    fn test_is_prime_td_for_composite() {
        assert!(is_prime_td(16) == false);
        assert!(is_prime_td(111) == false);
        assert!(is_prime_td(1729) == false);
        assert!(is_prime_td(8679) == false);
    }

    #[test]
    fn test_cannot_be_prime_td_for_primes() {
        assert!(cannot_be_prime_td(97) == false);
        assert!(cannot_be_prime_td(101) == false);
        assert!(cannot_be_prime_td(103) == false);
        assert!(cannot_be_prime_td(1217) == false);
    }

    #[test]
    fn test_cannot_be_prime_td_for_composite() {
        assert!(cannot_be_prime_td(1729) == true);
        assert!(cannot_be_prime_td(8321) == true);
        assert!(cannot_be_prime_td(31697) == true);
        // following has too large factors (163,487) to get verification
        assert!(cannot_be_prime_td(79381) == false);
    }

    #[test]
    fn test_is_prime_for_small_primes_mr() {
        assert!(is_prime_mr(1009) == true);
        assert!(is_prime_mr(1709) == true);
        assert!(is_prime_mr(99_991) == true);
        assert!(is_prime_mr(15_485_863) == true);
    }

    #[test]
    fn test_is_prime_for_mid_primes_mr() {
        assert!(is_prime_mr(256_203_221) == true);
        assert!(is_prime_mr(633_910_099) == true);
        assert!(is_prime_mr(982_451_653) == true);
    }

    #[test]
    fn test_is_prime_for_large_primes_mr() {
        assert!(is_prime_mr(4294967291) == true);
        assert!(is_prime_mr(50000038603) == true);
        assert!(is_prime_mr(9223372036854775337) == true);
        assert!(is_prime_mr(9223372036854775783) == true);
    }

    #[test]
    fn test_is_prime_for_composite_mr() {
        assert!(is_prime_mr(1729) == false);
        assert!(is_prime_mr(13021) == false);
        assert!(is_prime_mr(79381) == false);
        assert!(is_prime_mr(25326001) == false);
        assert!(is_prime_mr(10449049901) == false);
        assert!(is_prime_mr(35127918407) == false);
        assert!(is_prime_mr(50131820635651) == false);
        assert!(is_prime_mr(25012804853117569) == false);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(17) == true);
        assert!(is_prime(10999) == false);
        assert!(is_prime(99991) == true);
        assert!(is_prime(1019387) == false);
        assert!(is_prime(35127918407) == false);
        assert!(is_prime(100159380682013) == false);
        assert!(is_prime(1002235870382890621) == false);
        assert!(is_prime(9223372036854775806) == false);
        assert!(is_prime(9223372036854775783) == true);
    }

    #[test]
    fn test_factorization_for_small_prime() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(2, &mut factors);

        let c_factors: [u64; 1] = [2];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_small_prime2() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(11, &mut factors);

        let c_factors: [u64; 1] = [11];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_small_composite() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(250, &mut factors);

        let c_factors: [u64; 4] = [2, 5, 5, 5];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_mid_prime() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(2147483647, &mut factors);

        let c_factors: [u64; 1] = [2147483647];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_mid_composite() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(534483647, &mut factors);

        let c_factors: [u64; 2] = [61, 8762027];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_large_prime() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(326326236253, &mut factors);

        let c_factors: [u64; 1] = [326326236253];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_large_composite() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(933720368547, &mut factors);

        let c_factors: [u64; 4] = [3, 191, 27191, 59929];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_large_composite_second() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(111729968547, &mut factors);

        let c_factors: [u64; 5] = [3, 17, 109, 1013, 19841];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_large_composite_third() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(614889782588491410, &mut factors);

        let c_factors: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_large_composite_fourth() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(18446744073709551615, &mut factors);

        let c_factors: [u64; 7] = [3, 5, 17, 257, 641, 65537, 6700417];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_large_composite_sixth() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(2342247720185763019, &mut factors);

        let c_factors: [u64; 5] = [7, 29, 257, 6700417, 6700417];
        let mut i = 0;

        for f in &factors {
            assert_eq!(*f, c_factors[i]);
            i += 1;
        }
    }

    #[test]
    fn test_factorization_for_power_of_prime() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(965211250482432409, &mut factors);

        assert!(factors.len() == 2);
        assert_eq!(factors[0], 982451653);
        assert_eq!(factors[1], 982451653);
    }

    #[test]
    fn test_factorization_for_power_of_prime_second() {
        let mut factors: Vec<u64> = Vec::new();
        factorize(4611686014132420609, &mut factors);

        assert!(factors.len() == 2);
        assert_eq!(factors[0], 2147483647);
        assert_eq!(factors[1], 2147483647);
    }
}
