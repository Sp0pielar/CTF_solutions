use std::convert::TryInto;

use crate::euclid;

pub fn quadratic_residue(a: i64, p: i64) -> i64 {
    if a == 0 {
        return 0;
    }

    let result_tuple: (i64, u64) =
        quadratic_residue_ts(a.try_into().unwrap(), p.try_into().unwrap());

    if result_tuple.0 == -1 {
        return -1;
    }

    (result_tuple.1).try_into().unwrap()
}

fn first_quadratic_nonresidue(p: u64) -> u64 {
    let mut z = 0;
    let mut j = 2;

    while z == 0 {
        if euclid::mod_exp_u64(j, (p - 1) / 2, p) != 1 {
            z = j;
        }
        j += 1;
    }
    z
}

fn quadratic_residue_ts(x: u64, p: u64) -> (i64, u64) {
    if euclid::mod_exp_u64(x, (p - 1) / 2, p) != 1 {
        let tuple = (-1, 0);
        return tuple;
    }
    let mut k = p - 1;
    let mut s = 0;

    while k & 1 == 0 {
        s += 1;
        k >>= 1;
    }
    let q = k;

    let z = first_quadratic_nonresidue(p);

    let mut m = s % p;

    let mut c = euclid::mod_exp_u64(z, q, p);
    let mut t = euclid::mod_exp_u64(x, q, p);
    let mut r = euclid::mod_exp_u64(x, (q + 1) / 2, p);

    let mut residue: (i64, u64) = (1, 0);

    loop {
        if t == 0 {
            residue.1 = 0;
            break;
        }
        if t == 1 {
            residue.1 = r;
            break;
        }
        let mut least_i = 0;
        for i in 1..m {
            let e = (1 << i) % p;
            if euclid::mod_exp_u64(t, e, p) == 1 {
                least_i = i;
                break;
            }
        }
        if least_i == 0 {
            residue.0 = -1;
            break;
        }
        let b = euclid::mod_exp_u64(c, (1 << (m - least_i - 1)) % p, p);

        m = least_i;
        c = euclid::mod_mult_u64(b, b, p);
        t = euclid::mod_mult_u64(t, c, p);
        r = euclid::mod_mult_u64(r, b, p);
    }

    residue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_residue_small() {
        let res = quadratic_residue_ts(1, 3);
        assert_eq!(res.0, 1);
        assert_eq!(res.1, 1);
    }

    #[test]
    fn test_quadratic_residue_small_second() {
        let res = quadratic_residue_ts(5, 41);
        assert_eq!(res.0, 1);
        assert_eq!(res.1, 28);
    }

    #[test]
    fn test_quadratic_residue_small_third() {
        let res = quadratic_residue_ts(99, 139);
        assert_eq!(res.0, 1);
        assert_eq!(res.1, 51);
    }

    #[test]
    fn test_quadratic_residue_no_solution_small() {
        let res = quadratic_residue_ts(209, 1223);
        assert!(res.0 == -1);
    }

    #[test]
    fn test_quadratic_residue_mid() {
        let res = quadratic_residue_ts(999, 14867);
        assert_eq!(res.0, 1);
        assert_eq!(res.1, 11699);
    }

    #[test]
    fn test_quadratic_residue_mid_second() {
        let res = quadratic_residue_ts(899, 50261);
        assert_eq!(res.0, 1);
        assert_eq!(res.1, 14696);
    }

    #[test]
    fn test_quadratic_residue_mid_third() {
        let res = quadratic_residue_ts(85155012, 65537);
        assert_eq!(res.0, 1);
        assert_eq!(res.1, 4688);
    }

    #[test]
    fn test_quadratic_residue_no_solution_mid() {
        let res = quadratic_residue_ts(1009990, 65537);
        assert!(res.0 == -1);
    }

    #[test]
    fn test_quadratic_residue_large() {
        let res = quadratic_residue_ts(100000, 50000038603);
        assert_eq!(res.0, 1);
        assert_eq!(res.1, 38257918326);
    }

    #[test]
    fn test_quadratic_residue_large_second() {
        let res = quadratic_residue_ts(999999999999, 9223372036854775337);
        assert_eq!(res.0, 1);
        assert_eq!(res.1, 5413383852200147669);
    }

    #[test]
    fn test_quadratic_residue_large_third() {
        let res = quadratic_residue_ts(9999999999999999, 9223372036854775783);
        assert_eq!(res.0, 1);
        assert_eq!(res.1, 8935381242731254940);
    }

    #[test]
    fn test_quadratic_residue_no_solution_large() {
        let res = quadratic_residue_ts(5, 9223372036854775783);
        assert!(res.0 == -1);
    }
}
