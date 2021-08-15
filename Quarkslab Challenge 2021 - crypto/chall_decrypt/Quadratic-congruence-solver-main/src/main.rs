use std::env;
use std::process;
use std::vec::Vec;

mod euclid;
mod prime;
mod residue;
mod solver;

pub const MAX_COEF_VAL: i64 = i64::MAX - 1;
pub const MIN_COEF_VAL: i64 = i64::MIN + 2;

fn help() {
    println!(
        "usage:
    quadsolver <a;integer> <b;integer> \
    <c;integer> <d;integer> <n;integer>
    Solve a quadratic congruence equation of the form \
    ax^2 + bx + c = d (mod n)."
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let equation = Equation::new(&args).unwrap_or_else(|err| {
        if err == "help" {
            process::exit(0);
        }
        println!("Error when parsing command line arguments: {}", err);
        process::exit(1);
    });

    solver::solve_equation(equation);
}

pub struct Coefs {
    pub a: i64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    pub n: i64,
}

pub enum Equation {
    Linear(Coefs),
    Quad(Coefs),
}

impl Equation {
    fn new(args: &[String]) -> Result<Equation, &str> {
        match args.len() {
            1 => Err("No arguments provided to solve equation of the form \
                ax^2 + bx + c = d (mod n)"),
            2 => {
                if &args[1] == "--help" {
                    help();
                    return Err("help");
                }
                Err(
                    "Not enough arguments provided to solve equation of the form \
                ax^2 + bx + c = d (mod n)",
                )
            }
            6 => {
                let mut coefs: Vec<i64> = Vec::new();
                match Equation::parse_to_integers(&mut coefs, &args) {
                    Err(x) => Err(x),
                    _ => match Equation::coef_values_are_valid(&coefs) {
                        Err(x) => Err(x),
                        _ => {
                            if coefs[0] == 0 {
                                Ok(Equation::Linear(Coefs {
                                    a: coefs[0],
                                    b: coefs[1],
                                    c: coefs[2],
                                    d: coefs[3],
                                    n: coefs[4],
                                }))
                            } else {
                                Ok(Equation::Quad(Coefs {
                                    a: coefs[0],
                                    b: coefs[1],
                                    c: coefs[2],
                                    d: coefs[3],
                                    n: coefs[4],
                                }))
                            }
                        }
                    },
                }
            }
            _ => Err(
                "Argument count mismatch in order to solve equation of the form \
                ax^2 + bx + c = d (mod n)",
            ),
        }
    }

    fn parse_to_integers(
        coefs: &mut Vec<i64>,
        args: &[String],
    ) -> Result<&'static str, &'static str> {
        for (i, arg) in (&args[1..]).iter().enumerate() {
            let num: i64 = match (*arg).parse() {
                Ok(x) => {
                    if i + 1 == args.len() - 1 {
                        if x <= 1 {
                            return Err("n must be larger than 1");
                        }
                        if x > MAX_COEF_VAL {
                            println!("warning: n must be smaller or equal than {}", MAX_COEF_VAL);
                            return Err("argument n not within bounds");
                        }
                    } else if x > MAX_COEF_VAL || x < MIN_COEF_VAL {
                        println!(
                            "warning: {}th argument must lie within interval \
                            [{}, {}]\n",
                            i + 1,
                            MIN_COEF_VAL,
                            MAX_COEF_VAL
                        );
                        return Err("argument not within bounds");
                    }
                    x
                }
                Err(_) => {
                    println!(
                        "warning: {}th argument not convertable to 64 bit signed integer\n",
                        i + 1
                    );
                    return Err("argument non-integer type");
                }
            };
            coefs.push(num);
        }
        Ok("parsing ok")
    }

    fn coef_values_are_valid(coefs: &Vec<i64>) -> Result<&'static str, &'static str> {
        if coefs[0] == 0 && coefs[1] == 0 {
            return Err("both `a` and `b` cannot be simultaneously zero");
        }
        if coefs[2] > 0 && coefs[3] < 0 || coefs[2] < 0 && coefs[3] > 0 {
            let c_abs = coefs[2].abs();
            let d_abs = coefs[3].abs();

            if c_abs >= i64::MAX - d_abs {
                return Err("abs(c+d) would overflow, cannot continue with \
                current argument values");
            }
        }
        Ok("coefs valid")
    }
}
