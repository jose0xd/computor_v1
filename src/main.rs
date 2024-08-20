use std::{collections::HashMap, env};

#[derive(Debug, PartialEq)]
enum ParseError {
    EqualSignError,
    ParseNumError,
}

struct Poly {
    coefficients: Vec<f32>,
}

impl Poly {
    pub fn new(line: &str) -> Result<Poly, ParseError> {
        let coefficients = parse(line)?;
        Ok(Poly { coefficients })
    }

    pub fn get_degree(&self) -> i32 {
        let degree: i32 = self.coefficients.len().try_into().unwrap();
        degree - 1
    }

    pub fn solve(&self) -> Option<Vec<f32>> {
        match self.get_degree() {
            0 => {
                if self.coefficients[0] == 0.0 {
                    Some(vec![])
                } else {
                    None
                }
            }
            1 => Some(vec![-self.coefficients[0] / self.coefficients[1]]),
            2 => self.quadratic_formula(),
            _ => None,
        }
    }

    fn quadratic_formula(&self) -> Option<Vec<f32>> {
        let a = self.coefficients[2];
        let b = self.coefficients[1];
        let c = self.coefficients[0];
        let discriminant = b * b - 4.0 * a * c;
        match discriminant {
            d if d > 0.0 => Some(vec![
                (-b + d.sqrt()) / (2.0 * a),
                (-b - d.sqrt()) / (2.0 * a),
            ]),
            d if d == 0.0 => Some(vec![-b / (2.0 * a)]),
            _ => None,
        }
    }

    pub fn print(&self) {
        print!("Reduced form: ");
        self.print_polinomial();
        println!(
            "Polynomial degree: {}",
            if self.get_degree() > -1 {
                self.get_degree()
            } else {
                0
            }
        );
        let solutions = self.solve();
        match self.get_degree() {
            0 => {
                if solutions.is_none() {
                    println!("There no solution")
                } else {
                    println!("Each real number is a solution")
                }
            }
            1 => println!("The solution is:\n{}", solutions.unwrap()[0]),
            2 => {
                if let Some(solutions) = solutions {
                    if solutions.len() == 1 {
                        println!(
                            "Discriminant is strictly zero, there is only one solution:\n{}",
                            solutions[0]
                        )
                    } else {
                        println!(
                            "Discriminant is strictly positive, the two solutions are:\n{}\n{}",
                            solutions[0], solutions[1]
                        )
                    }
                } else {
                    println!("Discriminant is strictly negative, there is no real solutions.")
                }
            }
            -1 => println!("Each real number is a solution."),
            _ => println!("The polynomial degree is strictly greater than 2, I can't solve."),
        }
    }

    fn print_polinomial(&self) {
        let mut degree = 0;
        while degree < self.coefficients.len() && self.coefficients[degree] == 0.0 {
            degree += 1
        }
        if degree < self.coefficients.len() {
            print!("{} * X^{}", self.coefficients[degree], degree);
        }
        degree += 1;
        while degree < self.coefficients.len() {
            if self.coefficients[degree] == 0.0 {
                degree += 1;
                continue;
            }
            if self.coefficients[degree] < 0.0 {
                print!(" - ")
            } else {
                print!(" + ")
            }
            print!("{} * X^{}", self.coefficients[degree].abs(), degree);
            degree += 1;
        }
        if self.coefficients.len() == 0 {
            print!("0");
        }
        println!(" = 0");
    }
}

fn parse(line: &str) -> Result<Vec<f32>, ParseError> {
    let line: String = line.chars().filter(|c| *c != ' ').collect(); // Remove spaces
    let equations: Vec<&str> = line.split('=').collect();
    if equations.len() != 2 {
        return Err(ParseError::EqualSignError);
    }
    let left_eq = parse_equation(equations[0])?;
    let right_eq = parse_equation(equations[1])?;
    let equation = simplify_equations(left_eq, right_eq);
    Ok(map2vec(equation))
}

fn parse_equation(equation: &str) -> Result<HashMap<i32, f32>, ParseError> {
    let equation = equation.replacen('-', "+-", equation.len());
    let monomial: Vec<&str> = equation.split('+').collect();
    let mut equation: HashMap<i32, f32> = HashMap::new();
    for m in monomial {
        match parse_monomial(m) {
            Ok((coef, degree)) => {
                if equation.contains_key(&degree) {
                    equation.insert(degree, coef + equation[&degree]);
                } else {
                    equation.insert(degree, coef);
                }
            },
            Err(_) => return Err(ParseError::ParseNumError),
        };
    }
    Ok(equation)
}

fn parse_monomial(monomial: &str) -> Result<(f32, i32), ParseError> {
    let elements: Vec<&str> = monomial.split('*').collect();
    if elements.len() == 2 {
        let coefficient = elements[0].parse::<f32>();
        let degree = parse_indeterminate(elements[1]);
        if let Ok(coefficient) = coefficient {
            if let Ok(degree) = degree {
                return Ok((coefficient, degree));
            }
        }
    } else if elements.len() == 1 && elements[0].contains('X') {
        let coefficient = 1.0;
        let degree = parse_indeterminate(elements[0]);
        if let Ok(degree) = degree {
            return Ok((coefficient, degree));
        }
    } else if elements.len() == 1 && elements[0].len() == 0 {
        return Ok((0., 0));
    } else {
        let coefficient = elements[0].parse::<f32>();
        let degree = 0;
        if let Ok(coefficient) = coefficient {
            return Ok((coefficient, degree));
        }
    }
    Err(ParseError::ParseNumError)
}

fn parse_indeterminate(indeterminate: &str) -> Result<i32, ParseError> {
    let exponentiation: Vec<&str> = indeterminate.split('^').collect();
    if exponentiation.len() == 2 && exponentiation[0].eq("X") {
        match exponentiation[1].parse::<i32>() {
            Ok(degree) => Ok(degree),
            _ => Err(ParseError::ParseNumError),
        }
    } else if exponentiation.len() == 1 && exponentiation[0].eq("X") {
        Ok(1)
    } else {
        Err(ParseError::ParseNumError)
    }
}

fn map2vec(map: HashMap<i32, f32>) -> Vec<f32> {
    let mut keys: Vec<&i32> = map.keys().collect();
    keys.sort();
    let mut vector: Vec<f32> = vec![];
    let mut i = 0;
    for k in keys {
        while i < *k {
            vector.push(0.0);
            i += 1;
        }
        vector.push(*map.get(k).unwrap());
        i += 1;
    }
    while !vector.is_empty() && vector[vector.len() - 1] == 0.0 {
        vector.pop();
    }
    vector
}

fn simplify_equations(
    left_eq: HashMap<i32, f32>,
    right_eq: HashMap<i32, f32>,
) -> HashMap<i32, f32> {
    let mut equation = left_eq;
    for (k, v) in right_eq {
        let monomial = equation.entry(k).or_insert(0.0);
        *monomial -= v;
    }
    equation
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("Wrong numbers of arguments");
        println!("Usage: ./computor \"5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0\"");
        return;
    }
    let poly = Poly::new(args.first().unwrap());
    if poly.is_err() {
        println!("Error parsing the polynomial equation");
        return;
    }
    let poly = poly.unwrap();
    poly.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn equivalent_solution(left: Vec<f32>, right: Vec<f32>) -> bool {
        if left.len() != right.len() {
            return false;
        }
        let wrong = left
            .iter()
            .zip(right)
            .filter(|&(a, b)| (a - b).abs() > 0.00001)
            .count();
        wrong == 0
    }

    #[test]
    fn error_when_no_equal_sign() {
        let no_equal_sign = "5 * X^0 + 4 * X^1 - 9.3 * X^2";
        assert!(parse(no_equal_sign).is_err());
    }

    #[test]
    fn parse_basic_monomial() {
        let basic_monomial = "5*X^0";
        assert_eq!(parse_monomial(basic_monomial), Ok((5.0, 0)));
    }

    #[test]
    fn test_parse_equation() {
        let line = "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0";
        let simplified = parse(line);
        let answer: Vec<f32> = vec![5.0, -6.0, 0.0, -5.6];
        assert_eq!(simplified, Ok(answer));
    }

    #[test]
    fn test_parse_bonus() {
        let line = "5 + 4 * X + X^2= X^2";
        let simplified = parse(line);
        let answer: Vec<f32> = vec![5.0, 4.0];
        assert_eq!(simplified, Ok(answer));
    }

    #[test]
    fn test_poly() {
        let line = "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0";
        let coefficients: Vec<f32> = vec![4.0, 4.0, -9.3];
        let poly = Poly::new(line).unwrap();
        assert_eq!(poly.coefficients, coefficients);
        assert_eq!(poly.get_degree(), 2);
    }

    #[test]
    fn test_solve() {
        let line = "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0";
        let poly = Poly::new(line).unwrap();
        let solutions = poly.solve().unwrap();
        assert!(equivalent_solution(solutions, vec![-0.475131, 0.905239]));

        let line = "5 * X^0 + 4 * X^1 = 4 * X^0";
        let poly = Poly::new(line).unwrap();
        let solutions = poly.solve().unwrap();
        assert!(equivalent_solution(solutions, vec![-0.25]));

        let line = "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0";
        let poly = Poly::new(line).unwrap();
        let solutions = poly.solve();
        assert_eq!(solutions, None);

        let line = "5 + 4 * X + X^2= X^2";
        let poly = Poly::new(line).unwrap();
        let solutions = poly.solve().unwrap();
        assert!(equivalent_solution(solutions, vec![-1.25]));

        let line = "42 * X^0= 42 * X^0";
        let poly = Poly::new(line).unwrap();
        let solutions = poly.solve();
        assert_eq!(solutions, None);

        let line = "3 = 0";
        let poly = Poly::new(line).unwrap();
        let solutions = poly.solve();
        assert_eq!(solutions, None);
    }
}
