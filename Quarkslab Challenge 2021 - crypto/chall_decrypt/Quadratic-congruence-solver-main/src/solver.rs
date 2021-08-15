use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::iter::FromIterator;
use std::vec::Vec;

use crate::Coefs;
use crate::Equation;

use crate::euclid;
use crate::prime;
use crate::residue;

pub fn solve_equation(equation: Equation) {
    match equation {
        Equation::Linear(mut coefs) => {
            let sol = solve_linear(&mut coefs);

            if sol.0 < 0 {
                println!("There is no solution in Z/{}Z", coefs.n);
            } else if sol.0 > 0 {
                println!("All solutions: {} + {}k (mod {})", sol.1, sol.0, coefs.n);
            } else {
                println!("All solutions: {} + {}k", sol.1, coefs.n);
            }
        }
        Equation::Quad(mut coefs) => {
            let sol = solve_quadratic(&mut coefs);

            let mut sol_exists = true;
            for s in &sol {
                if *s == -1 {
                    sol_exists = false;
                    break;
                }
            }
            if sol_exists {
                println!("Solutions x in Z/{}Z", coefs.n);
                for (j, x) in (&sol).iter().enumerate() {
                    println!("x_{}: {}", j + 1, *x);
                }
            } else {
                println!("There is no solution in Z/{}Z", coefs.n);
            }
        }
    }
}

fn add_until_nonnegative(x: i64, m: i64) -> i64 {
    // find smallest k : km >= x, for x < 0
    let abs_x = i64::abs(x);
    if abs_x <= m {
        return x + m;
    }

    let mut k = abs_x / m;
    if abs_x % m > 0 {
        k += 1;
    }

    k * m - abs_x
}

pub fn solve_linear(coefs: &mut Coefs) -> (i64, i64) {
    coefs.d -= coefs.c;
    if coefs.b < 0 {
        coefs.b = add_until_nonnegative(coefs.b, coefs.n);
    }
    if coefs.d < 0 {
        coefs.d = add_until_nonnegative(coefs.d, coefs.n);
    }

    linear_eq(&coefs)
}

fn linear_eq(coefs: &Coefs) -> (i64, i64) {
    let gcd_bn: i64 = euclid::gcd(coefs.n, coefs.b);
    let mut sol: (i64, i64) = (0, 0);

    if coefs.d % gcd_bn != 0 {
        sol.0 = -1; // no solution
        return sol;
    }

    if gcd_bn == 1 {
        sol.1 = {
            euclid::mod_mult_i64(euclid::multip_inverse(coefs.b, coefs.n), coefs.d, coefs.n)
                % coefs.n
        };
    } else {
        let n_d = coefs.n / gcd_bn;

        sol.1 = {
            euclid::mod_mult_i64(
                euclid::multip_inverse(coefs.b / gcd_bn, n_d),
                coefs.d / gcd_bn,
                n_d,
            ) % n_d
        };
        sol.0 = n_d;
    }
    sol
}

pub fn solve_quadratic(mut coefs: &mut Coefs) -> Vec<i64> {
    coefs.d -= coefs.c;

    if coefs.a < 0 {
        coefs.a = add_until_nonnegative(coefs.a, coefs.n);
    }
    if coefs.b < 0 {
        coefs.b = add_until_nonnegative(coefs.b, coefs.n);
    }
    if coefs.d < 0 {
        coefs.d = add_until_nonnegative(coefs.d, coefs.n);
    }

    if prime::is_prime(coefs.n) && coefs.n > 2 {
        // (2ax + b)^2 = b^2 + 4ad (mod n), n>2
        let x_l = euclid::mod_mult_i64(coefs.b, coefs.b, coefs.n);
        let x_r = euclid::mod_mult_i64(
            4i64,
            euclid::mod_mult_i64(coefs.a, coefs.d, coefs.n),
            coefs.n,
        );
        let rhs = euclid::mod_sum_i64(x_l, x_r, coefs.n);

        return quadratic_eq(&coefs, rhs);
    }

    let mut factors: Vec<u64> = Vec::new();
    let n: u64 = (coefs.n).try_into().unwrap();
    prime::factorize(n, &mut factors);

    let mut factor_map: HashMap<u64, i64> = HashMap::new();

    for factor in factors.iter() {
        let count = factor_map.entry(*factor).or_insert(0);
        *count += 1;
    }

    quadratic_eq_composite_mod(&mut coefs, factor_map)
}

fn quadratic_eq_composite_mod(coefs: &mut Coefs, factor_map: HashMap<u64, i64>) -> Vec<i64> {
    let n_orig = coefs.n;
    let mut sols: Vec<(i64, i64)> = Vec::new();

    let mut diff_factor_count = 0;

    for (f, c) in &factor_map {
        coefs.n = (*f).try_into().unwrap();
        let c_u32: u32 = (*c).try_into().unwrap(); // c < 64
        let modulo = i64::pow(coefs.n, c_u32);

        diff_factor_count += 1;

        let sub_sols = if coefs.n == 2 {
            quadratic_eq_mod_two(&coefs, c_u32)
        } else {
            let l = euclid::mod_mult_i64(coefs.b, coefs.b, coefs.n);
            let r = euclid::mod_mult_i64(
                4i64,
                euclid::mod_mult_i64(coefs.a, coefs.d, coefs.n),
                coefs.n,
            );
            let rhs = euclid::mod_sum_i64(l, r, coefs.n);

            let sub_sols = quadratic_eq(&coefs, rhs);
            for j in &sub_sols {
                if *j == -1 {
                    coefs.n = n_orig;
                    return vec![-1];
                }
            }
            lift_with_hensels_method(&coefs, sub_sols, c_u32)
        };

        if sub_sols.is_empty() {
            coefs.n = n_orig;
            return vec![-1];
        }
        for j in sub_sols {
            sols.push((j, modulo));
        }
    }
    coefs.n = n_orig;

    let x: Vec<i64> = if diff_factor_count > 1 {
        euclid::crt(sols, coefs.n)
    } else {
        let mut x_t: Vec<i64> = Vec::new();
        for j in &sols {
            x_t.push((*j).0);
        }
        x_t
    };
    x
}

fn quadratic_eq(coefs: &Coefs, rhs: i64) -> Vec<i64> {
    let mut sols: Vec<i64> = Vec::new();
    // (2ax + b)^2 = z^2 = rhs = b^2 + 4ad (mod n)
    let z = residue::quadratic_residue(rhs, coefs.n);
    if z < 0 {
        sols.push(-1); // rhs not a quadratic residue => no solutions
        return sols;
    }

    let mut d = z - coefs.b;
    if d < 0 {
        d = add_until_nonnegative(d, coefs.n);
    }

    let b = euclid::mod_mult_i64(2i64, coefs.a, coefs.n);

    let mut lin_coefs = Coefs {
        a: 0,
        b,
        c: 0,
        d,
        n: coefs.n,
    };

    let sol = linear_eq(&lin_coefs);
    if sol.0 < 0 {
        sols.push(-1); // no solutions
        return sols;
    }
    sols.push(sol.1);
    if z == 0 {
        return sols;
    }

    d = -z;
    if d < 0 {
        d = add_until_nonnegative(d, coefs.n);
    }
    d -= coefs.b;
    if d < 0 {
        d = add_until_nonnegative(d, coefs.n);
    }
    lin_coefs.d = d;

    let sol = linear_eq(&lin_coefs);
    sols.push(sol.1);

    sols
}

fn quadratic_eq_mod_two(coefs: &Coefs, c: u32) -> Vec<i64> {
    match coefs.b {
        0 => quadratic_residue_mod_pow_of_two(&coefs, c),
        _ => {
            if coefs.d % 2 != 0 {
                if coefs.a % 2 != 0 && coefs.b % 2 != 0 {
                    return vec![];
                }
                if coefs.a % 2 == 0 && coefs.b % 2 == 0 {
                    return vec![];
                }
            }
            let sub_sols = quadratic_mod_general_pow_of_two(&coefs);
            if sub_sols.is_empty() {
                return sub_sols;
            }
            let t: u32 = euclid::largest_common_dividing_power_of_two((coefs.a, coefs.b), coefs.d);

            let m_coefs = Coefs {
                a: coefs.a >> t,
                b: coefs.b >> t,
                c: coefs.c,
                d: coefs.d >> t,
                n: coefs.n,
            };
            let m_c: u32 = c - t; // >= 0

            let sub_sols = lift_with_hensels_method(&m_coefs, sub_sols, m_c);
            if sub_sols.is_empty() {
                return sub_sols;
            }

            if t == 0 {
                sub_sols
            } else {
                let modulo = i64::pow(coefs.n, c);
                let mod_t = i64::pow(coefs.n, t);
                let multiplier = i64::pow(coefs.n, m_c);

                let mut sols = HashSet::new();

                for x in &sub_sols {
                    let mut r = 0;
                    while r < mod_t {
                        sols.insert(euclid::mod_sum_i64(
                            *x,
                            euclid::mod_mult_i64(r, multiplier, modulo),
                            modulo,
                        ));
                        r += 1;
                    }
                }
                Vec::from_iter(sols)
            }
        }
    }
}

fn quadratic_residue_mod_pow_of_two(coefs: &Coefs, c: u32) -> Vec<i64> {
    let n = i64::pow(coefs.n, c);

    match c {
        1 => quadratic_residue_mod_two(&coefs),
        2 => quadratic_residue_mod_four(&coefs, n),
        _ => {
            if euclid::gcd(n, coefs.a) == 1 {
                quadratic_residue_mod_higher_power(&coefs, n, c)
            } else if coefs.a & 1 == 0 && coefs.d & 1 == 0 && coefs.a > 0 {
                if coefs.a % n == 0 {
                    vec![]
                } else {
                    quadratic_residue_even_terms_mod_higher_power(&coefs, n, c)
                }
            } else {
                vec![]
            }
        }
    }
}

fn quadratic_residue_even_terms_mod_higher_power(coefs: &Coefs, n: i64, c: u32) -> Vec<i64> {
    let t: u32 = euclid::largest_common_dividing_power_of_two((coefs.a % n, 2), coefs.d);
    let m_c: u32 = c - t; // >= 0, t >= 1

    let sub_sols: Vec<i64> = if coefs.d > 0 {
        let m_coefs = Coefs {
            a: coefs.a >> t,
            b: coefs.b,
            c: coefs.c,
            d: coefs.d >> t,
            n: coefs.n,
        };
        let n_c = i64::pow(coefs.n, m_c);

        match m_c {
            0 => vec![],
            1 => quadratic_residue_mod_two(&m_coefs),
            2 => quadratic_residue_mod_four(&m_coefs, n_c),
            _ => {
                if euclid::gcd(n_c, coefs.a) == 1 {
                    quadratic_residue_mod_higher_power(&m_coefs, n_c, m_c)
                } else {
                    vec![]
                }
            }
        }
    } else {
        vec![0]
    };

    if sub_sols.is_empty() {
        return sub_sols;
    }

    let modulo = i64::pow(coefs.n, c);
    let mod_t = i64::pow(coefs.n, t);
    let multiplier = i64::pow(coefs.n, m_c);

    let mut sols = HashSet::new();

    for x in &sub_sols {
        let mut r = 0;
        while r < mod_t {
            sols.insert(euclid::mod_sum_i64(
                *x,
                euclid::mod_mult_i64(r, multiplier, modulo),
                modulo,
            ));
            r += 1;
        }
    }
    Vec::from_iter(sols)
}

fn quadratic_residue_mod_two(coefs: &Coefs) -> Vec<i64> {
    match coefs.d & 1 {
        0 => {
            if coefs.a & 1 != 0 {
                vec![0]
            } else {
                vec![0, 1]
            }
        }
        _ => {
            if coefs.a & 1 != 0 {
                vec![1]
            } else {
                vec![]
            }
        }
    }
}

fn quadratic_residue_mod_four(coefs: &Coefs, n: i64) -> Vec<i64> {
    match coefs.d & 1 {
        0 => match coefs.a % 4 {
            0 => {
                if coefs.d % 4 == 0 {
                    vec![0, 1, 2, 3]
                } else {
                    vec![]
                }
            }
            2 => {
                if coefs.d % 4 == 0 {
                    vec![0, 2]
                } else {
                    vec![1, 3]
                }
            }
            _ => {
                if coefs.d % 4 == 0 {
                    vec![0, 2]
                } else {
                    vec![]
                }
            }
        },
        _ => {
            if euclid::gcd(n, coefs.a) == 1 {
                let inv = euclid::multip_inverse(coefs.a, n);
                let d = euclid::mod_mult_i64(inv, coefs.d, n);
                if d % 4 == 1 {
                    vec![1, 3]
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        }
    }
}

fn quadratic_residue_mod_higher_power(coefs: &Coefs, n: i64, c: u32) -> Vec<i64> {
    let inv = euclid::multip_inverse(coefs.a, n);
    let d = euclid::mod_mult_i64(inv, coefs.d, n);

    if d == 0 {
        let mut sols: Vec<i64> = vec![0];
        let t: u32 = (c as f64 / 2f64).ceil() as u32; // c < 64

        sols.push(i64::pow(coefs.n, t));
        return sols;
    }

    match d % 8 {
        1 => {
            let mut x: Vec<i64> = Vec::new();
            let sols: Vec<i64> = vec![1, 3];

            for i in sols.into_iter() {
                let mut s = i;

                for j in 3..c {
                    let t = i64::pow(2, j);
                    let r = i64::abs(euclid::mod_mult_i64(s, s, n) - d) / t;
                    s = euclid::mod_sum_i64(s, (r % 2) * (t / 2), n);
                }
                x.push(s);
                x.push(n - s);
            }
            x
        }
        _ => {
            vec![]
        }
    }
}

fn quadratic_mod_general_pow_of_two(coefs: &Coefs) -> Vec<i64> {
    let mut sols: Vec<i64> = Vec::new();

    let s_cand: Vec<i64> = vec![0, 1];

    for s in s_cand.into_iter() {
        let f_lhs = euclid::mod_sum_i64(
            euclid::mod_mult_i64(coefs.a, s * s, coefs.n),
            coefs.b * s,
            coefs.n,
        );
        if f_lhs % coefs.n == coefs.d % coefs.n {
            sols.push(s);
        }
    }
    sols
}

fn lift_with_hensels_method(coefs: &Coefs, sub_sols: Vec<i64>, c: u32) -> Vec<i64> {
    if c <= 1 {
        return sub_sols;
    }
    let mut sols: Vec<i64> = Vec::new();

    for s in &sub_sols {
        let d_x = euclid::mod_mult_i64(2i64, euclid::mod_mult_i64(coefs.a, *s, coefs.n), coefs.n);
        let deriv = euclid::mod_sum_i64(d_x, coefs.b % coefs.n, coefs.n);

        if euclid::gcd(coefs.n, deriv) != 1 {
            continue; // singular root, cannot lift
        }
        let t = euclid::multip_inverse(deriv, coefs.n);

        let mut n = coefs.n;
        let mut lifted_s = *s;

        for _j in 1..c {
            n *= coefs.n;

            let ax = euclid::mod_mult_i64(coefs.a, euclid::mod_mult_i64(lifted_s, lifted_s, n), n);
            let bx = euclid::mod_mult_i64(coefs.b, lifted_s, n);

            let mut cx = -coefs.d;
            if cx < 0 {
                cx = add_until_nonnegative(cx, n);
            }

            let func = euclid::mod_sum_i64(euclid::mod_sum_i64(ax, bx, n), cx, n);
            let m = euclid::mod_mult_i64(func, t, n);

            lifted_s -= m;
            if lifted_s < 0 {
                lifted_s = add_until_nonnegative(lifted_s, n);
            }
        }
        sols.push(lifted_s);
    }
    sols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_solver_small() {
        let mut coefs = Coefs {
            a: 0,
            b: 2,
            c: 0,
            d: 3,
            n: 5,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 0);
        assert_eq!(res.1, 4);
    }

    #[test]
    fn test_linear_solver_small_no_solution() {
        let mut coefs = Coefs {
            a: 0,
            b: 2,
            c: 0,
            d: 3,
            n: 8,
        };
        let res = solve_linear(&mut coefs);

        assert!(res.0 == -1);
    }

    #[test]
    fn test_linear_solver_small_second() {
        let mut coefs = Coefs {
            a: 0,
            b: 21,
            c: 0,
            d: 89,
            n: 457,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 0);
        assert_eq!(res.1, 26);
    }

    #[test]
    fn test_linear_solver_small_with_neg_additive_part() {
        let mut coefs = Coefs {
            a: 0,
            b: 21,
            c: -11,
            d: 78,
            n: 457,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 0);
        assert_eq!(res.1, 26);
    }

    #[test]
    fn test_linear_solver_small_with_pos_additive_part() {
        let mut coefs = Coefs {
            a: 0,
            b: 199,
            c: 11598,
            d: 7815,
            n: 1723,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 0);
        assert_eq!(res.1, 1349);
    }

    #[test]
    fn test_linear_solver_small_with_non_coprime_pair_bn() {
        let mut coefs = Coefs {
            a: 0,
            b: 5,
            c: 0,
            d: 7815,
            n: 1725,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 345);
        assert_eq!(res.1, 183);
    }

    #[test]
    fn test_linear_solver_small_with_non_coprime_pair_bn_and_neg_b() {
        let mut coefs = Coefs {
            a: 0,
            b: -55,
            c: -55,
            d: 65,
            n: 105,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 21);
        assert_eq!(res.1, 15);
    }

    #[test]
    fn test_linear_solver_large() {
        let mut coefs = Coefs {
            a: 0,
            b: -9832503233,
            c: 235232447,
            d: 653245724,
            n: 7919,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 0);
        assert_eq!(res.1, 1395);
    }

    #[test]
    fn test_linear_solver_large_no_solution() {
        let mut coefs = Coefs {
            a: 0,
            b: -23850975512223,
            c: -90003424242,
            d: -902412412,
            n: 9223372036854775782,
        };
        let res = solve_linear(&mut coefs);

        assert!(res.0 == -1);
    }

    #[test]
    fn test_linear_solver_large_second() {
        let mut coefs = Coefs {
            a: 0,
            b: -9832503233,
            c: 235232447,
            d: 653245724,
            n: 50131820635651,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 0);
        assert_eq!(res.1, 44363860600404);
    }

    #[test]
    fn test_linear_solver_large_third() {
        let mut coefs = Coefs {
            a: 0,
            b: 2,
            c: 23523244703424242,
            d: 653245724232,
            n: 9223372036854775783,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 0);
        assert_eq!(res.1, 9211610741125925778);
    }

    #[test]
    fn test_linear_solver_large_with_additive_part() {
        let mut coefs = Coefs {
            a: 0,
            b: -55235,
            c: 5520,
            d: 653245,
            n: 555,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 111);
        assert_eq!(res.1, 19);
    }

    #[test]
    fn test_linear_solver_large_with_non_coprime_pair_bn() {
        let mut coefs = Coefs {
            a: 0,
            b: 2,
            c: 23523244703424242,
            d: 653245724232,
            n: 9223372036854775782,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 4611686018427387891);
        assert_eq!(res.1, 4599924722698537886);
    }

    #[test]
    fn test_linear_solver_large_with_neg_b() {
        let mut coefs = Coefs {
            a: 0,
            b: -999999999997,
            c: 91922559,
            d: 902412412,
            n: 9223372036854775782,
        };
        let res = solve_linear(&mut coefs);

        assert_eq!(res.0, 0);
        assert_eq!(res.1, 5344334800772456633);
    }

    #[test]
    fn test_quadratic_solver_small_prime_modulus() {
        let mut coefs = Coefs {
            a: 3,
            b: 6,
            c: 1,
            d: 0,
            n: 19,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&7));
        assert!(res.contains(&10));
    }

    #[test]
    fn test_quadratic_solver_small_prime_modulus_second() {
        let mut coefs = Coefs {
            a: 3,
            b: 6,
            c: 0,
            d: 18,
            n: 19,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&7));
        assert!(res.contains(&10));
    }

    #[test]
    fn test_quadratic_solver_small_prime_modulus_third() {
        let mut coefs = Coefs {
            a: 1,
            b: 1,
            c: 0,
            d: 0,
            n: 7,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&0));
        assert!(res.contains(&6));
    }

    #[test]
    fn test_quadratic_solver_prime_modulus() {
        let mut coefs = Coefs {
            a: 1,
            b: 1,
            c: 0,
            d: 2,
            n: 3,
        };
        let res = solve_quadratic(&mut coefs);

        assert!(res.len() == 1);
        assert_eq!(res[0], 1);
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_second() {
        let mut coefs = Coefs {
            a: 1,
            b: 1,
            c: 5,
            d: 0,
            n: 11,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&2));
        assert!(res.contains(&8));
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_third() {
        let mut coefs = Coefs {
            a: 5,
            b: 71,
            c: 600,
            d: 1,
            n: 1709,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&144));
        assert!(res.contains(&1209));
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_fourth() {
        let mut coefs = Coefs {
            a: -9138,
            b: 51252,
            c: 18000,
            d: 6000,
            n: 99991,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&267));
        assert!(res.contains(&63226));
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_fifth() {
        let mut coefs = Coefs {
            a: 2,
            b: 8,
            c: 2,
            d: 0,
            n: 23,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&5));
        assert!(res.contains(&14));
    }

    #[test]
    fn test_quadratic_solver_residue_case() {
        let mut coefs = Coefs {
            a: 1,
            b: 0,
            c: 0,
            d: 0,
            n: 9223372036854775783,
        };
        let res = solve_quadratic(&mut coefs);

        assert!(res.len() == 1);
        assert_eq!(res[0], 0);
    }

    #[test]
    fn test_quadratic_solver_residue_case_second() {
        let mut coefs = Coefs {
            a: 1,
            b: 0,
            c: 0,
            d: 9999999999999999,
            n: 9223372036854775783,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&287990794123520843));
        assert!(res.contains(&8935381242731254940));
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_double_root() {
        let mut coefs = Coefs {
            a: 3,
            b: -5,
            c: 0,
            d: 2,
            n: 7,
        };
        let res = solve_quadratic(&mut coefs);

        assert!(res.len() == 1);
        assert_eq!(res[0], 2);
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_neg_coefs() {
        let mut coefs = Coefs {
            a: -999999,
            b: -333333,
            c: 0,
            d: -93939393,
            n: 99991,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&220));
        assert!(res.contains(&33110));
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_large_prime_mod() {
        let mut coefs = Coefs {
            a: 12381221,
            b: -21212322,
            c: 92013811,
            d: 1,
            n: 4294967291,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&97664718));
        assert!(res.contains(&2483511823));
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_large_prime_mod_second() {
        let mut coefs = Coefs {
            a: 1212421490235,
            b: 91595920724124,
            c: 0,
            d: 74825828142,
            n: 9223372036854775783,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&3586932499142287740));
        assert!(res.contains(&6079892515866449701));
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_large_prime_mod_third() {
        let mut coefs = Coefs {
            a: 1,
            b: 0,
            c: 0,
            d: 9999999999999999,
            n: 9223372036854775783,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&287990794123520843));
        assert!(res.contains(&8935381242731254940));
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_no_solution_first() {
        let mut coefs = Coefs {
            a: 5,
            b: 1,
            c: 8,
            d: 0,
            n: 11,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 1);
        assert!(res[0] == -1);
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_no_solution_second() {
        let mut coefs = Coefs {
            a: 1,
            b: 0,
            c: 0,
            d: 13961,
            n: 50261,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 1);
        assert!(res[0] == -1);
    }

    #[test]
    fn test_quadratic_solver_prime_modulus_no_solution_third() {
        let mut coefs = Coefs {
            a: 1238122191,
            b: -212897922212924,
            c: -52552550051514,
            d: 0,
            n: 9223372036854775783,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 1);
        assert!(res[0] == -1);
    }

    #[test]
    fn test_quadratic_solver_modulus_two() {
        let mut coefs = Coefs {
            a: 5,
            b: 1,
            c: 8,
            d: 0,
            n: 2,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&0));
        assert!(res.contains(&1));
    }

    #[test]
    fn test_quadratic_solver_modulus_two_second() {
        let mut coefs = Coefs {
            a: 1,
            b: 1,
            c: 4,
            d: 0,
            n: 2,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&0));
        assert!(res.contains(&1));
    }

    #[test]
    fn test_quadratic_solver_modulus_two_third() {
        let mut coefs = Coefs {
            a: 1,
            b: 0,
            c: 3,
            d: 0,
            n: 2,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 1);
        assert_eq!(res[0], 1);
    }

    #[test]
    fn test_quadratic_solver_modulus_two_no_solution() {
        let mut coefs = Coefs {
            a: 1,
            b: 1,
            c: 1,
            d: 0,
            n: 2,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 1);
        assert!(res[0] == -1);
    }

    #[test]
    fn test_quadratic_solver_modulus_two_no_solution_second() {
        let mut coefs = Coefs {
            a: 8,
            b: 0,
            c: 3,
            d: 0,
            n: 2,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 1);
        assert!(res[0] == -1);
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two() {
        let mut coefs = Coefs {
            a: 7,
            b: 0,
            c: 1,
            d: 0,
            n: 4,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&1));
        assert!(res.contains(&3));
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_second() {
        let mut coefs = Coefs {
            a: 1,
            b: 0,
            c: 31,
            d: 0,
            n: 4,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&1));
        assert!(res.contains(&3));
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_third() {
        let mut coefs = Coefs {
            a: 3,
            b: 0,
            c: 29,
            d: 0,
            n: 4,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&1));
        assert!(res.contains(&3));
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_fourth() {
        let mut coefs = Coefs {
            a: 8,
            b: 0,
            c: 12,
            d: 0,
            n: 4,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 4);
        assert!(res.contains(&0));
        assert!(res.contains(&1));
        assert!(res.contains(&2));
        assert!(res.contains(&3));
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_fifth() {
        let mut coefs = Coefs {
            a: 6,
            b: 0,
            c: 12,
            d: 0,
            n: 4,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&0));
        assert!(res.contains(&2));
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_sixth() {
        let mut coefs = Coefs {
            a: 6,
            b: 0,
            c: 10,
            d: 0,
            n: 4,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 2);
        assert!(res.contains(&1));
        assert!(res.contains(&3));
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_seventh() {
        let mut coefs = Coefs {
            a: 7,
            b: 0,
            c: 1,
            d: 0,
            n: 4096,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.len() == 4);
        assert!(res.contains(&611));
        assert!(res.contains(&1437));
        assert!(res.contains(&2659));
        assert!(res.contains(&3485));
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_eigth() {
        let mut coefs = Coefs {
            a: 1,
            b: 0,
            c: 15,
            d: 0,
            n: 4294967296,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        let correct_res: Vec<i64> = vec![34716455, 2112767193, 2182200103, 4260250841];

        for r in &correct_res {
            assert!(res.contains(&*r));
        }
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_ninth() {
        let mut coefs = Coefs {
            a: 1,
            b: 0,
            c: 25151551,
            d: 0,
            n: 4611686018427387904,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        let correct_res: Vec<i64> = vec![
            949829031310219745,
            1356013977903474207,
            3255672040523913697,
            3661856987117168159,
        ];

        for r in &correct_res {
            assert!(res.contains(&*r));
        }
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_tenth() {
        let mut coefs = Coefs {
            a: 999999999999999999,
            b: 0,
            c: 1,
            d: 0,
            n: 4611686018427387904,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        let correct_res: Vec<i64> = vec![
            567654762933256193,
            1738188246280437759,
            2873497772146950145,
            4044031255494131711,
        ];

        for r in &correct_res {
            assert!(res.contains(&*r));
        }
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_special_even_case() {
        let mut coefs = Coefs {
            a: 4,
            b: 4,
            c: 24,
            d: 0,
            n: 32,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        let correct_res: Vec<i64> = vec![1, 6, 9, 14, 17, 22, 25, 30];

        for r in &correct_res {
            assert!(res.contains(&*r));
        }
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_two_special_even_case_second() {
        let mut coefs = Coefs {
            a: 64,
            b: 64,
            c: 0,
            d: 0,
            n: 256,
        };
        let res = solve_quadratic(&mut coefs);
        assert_eq!(res.len(), 128);
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_prime() {
        let mut coefs = Coefs {
            a: 1,
            b: 1,
            c: 47,
            d: 0,
            n: 343,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.contains(&99));
        assert!(res.contains(&243));
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_prime_second() {
        let mut coefs = Coefs {
            a: 999999999999999999,
            b: -999999999912421,
            c: 214081248358023524,
            d: 0,
            n: 2862423051509815793,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.contains(&2303508973012165250));
        assert!(res.contains(&2721119028450610552));
    }

    #[test]
    fn test_quadratic_solver_modulus_power_of_prime_third() {
        let mut coefs = Coefs {
            a: -125125121242124,
            b: -54224212353523,
            c: 113535124124255,
            d: 0,
            n: 4611686014132420609,
        };
        let res = solve_quadratic(&mut coefs);
        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.contains(&1523832291260501430));
        assert!(res.contains(&2424331690299886142));
    }

    #[test]
    fn test_quadratic_solver_composite_modulus() {
        let mut coefs = Coefs {
            a: 1,
            b: 3,
            c: 0,
            d: 298,
            n: 315,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 8);

        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.contains(&218));
        assert!(res.contains(&38));
        assert!(res.contains(&29));
        assert!(res.contains(&164));
        assert!(res.contains(&148));
        assert!(res.contains(&283));
        assert!(res.contains(&274));
        assert!(res.contains(&94));
    }

    #[test]
    fn test_quadratic_solver_composite_modulus_second() {
        let mut coefs = Coefs {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
            n: 72,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 8);

        let res: HashSet<i64> = HashSet::from_iter(res);

        assert!(res.contains(&1));
        assert!(res.contains(&17));
        assert!(res.contains(&19));
        assert!(res.contains(&35));
        assert!(res.contains(&37));
        assert!(res.contains(&53));
        assert!(res.contains(&55));
        assert!(res.contains(&71));
    }

    #[test]
    fn test_quadratic_solver_composite_modulus_third() {
        let mut coefs = Coefs {
            a: 1,
            b: 3124213,
            c: 1231121313123,
            d: 0,
            n: 9223372036854775803,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 16);

        let res: HashSet<i64> = HashSet::from_iter(res);

        // test a few
        assert!(res.contains(&566238308012032964));
        assert!(res.contains(&8657133728839618626));
    }

    #[test]
    fn test_quadratic_solver_composite_modulus_fourth() {
        let mut coefs = Coefs {
            a: -999999999999991320,
            b: -911191191232131242,
            c: -982481241441241120,
            d: 0,
            n: 9223372036854775798,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 8);

        let res: HashSet<i64> = HashSet::from_iter(res);

        let correct_res: Vec<i64> = vec![
            3204775211312642198,
            6343870335019630392,
            9026692187714946424,
            2942415274567158820,
            7816461229740030097,
            1732184316592242493,
            4415006169287558525,
            7554101292994546719,
        ];

        for r in &correct_res {
            assert!(res.contains(&*r));
        }
    }

    #[test]
    fn test_quadratic_solver_composite_modulus_fifth() {
        let mut coefs = Coefs {
            a: 2,
            b: 3,
            c: 5,
            d: 0,
            n: 5000000000000000000,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 2);

        let res: HashSet<i64> = HashSet::from_iter(res);

        let correct_res: Vec<i64> = vec![966704601316436515, 2719160800905243171];

        for r in &correct_res {
            assert!(res.contains(&*r));
        }
    }

    #[test]
    fn test_quadratic_solver_composite_modulus_sixth() {
        let mut coefs = Coefs {
            a: 2,
            b: 2,
            c: 0,
            d: 0,
            n: 1000114242141100000,
        };
        let res = solve_quadratic(&mut coefs);
        assert!(res.len() == 32);

        let res: HashSet<i64> = HashSet::from_iter(res);

        // test a few
        assert!(res.contains(&0));
        assert!(res.contains(&1000114242141099999));
        assert!(res.contains(&843917841431300000));
    }

    #[test]
    fn test_quadratic_solver_composite_modulus_seventh() {
        let mut coefs = Coefs {
            a: 1,
            b: 2,
            c: 1,
            d: 0,
            n: 614889782588491410, // n = multiple of all primes from 2 to 47
        };
        let res = solve_quadratic(&mut coefs);

        assert!(res.len() == 1);
        assert_eq!(res[0], 614889782588491409);
    }
}
