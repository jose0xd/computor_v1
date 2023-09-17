use std::{env, collections::HashMap};

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

    pub fn get_degree(&self) -> usize { return self.coefficients.len() - 1 }
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
    let equation = equation.replacen("-", "+-", equation.len());
    let monomial: Vec<&str> = equation.split('+').collect();
    let mut equation: HashMap<i32, f32> = HashMap::new();
    for m in monomial {
        match parse_monomial(m) {
            Ok((coef, degree)) => equation.insert(degree, coef),
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
        if coefficient.is_ok() && degree.is_ok() {
            return Ok((coefficient.unwrap(), degree.unwrap()));
        }
    } else if elements.len() == 1 && elements[0].contains("X") {
        let coefficient = 1.0;
        let degree = parse_indeterminate(elements[0]);
        if degree.is_ok() {
            return Ok((coefficient, degree.unwrap()));
        }
    } else {
        let coefficient = elements[0].parse::<f32>();
        let degree = 0;
        if coefficient.is_ok() {
            return Ok((coefficient.unwrap(), degree));
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

fn map2vec(map: HashMap<i32, f32>) -> Vec<f32> { // TODO refactor
    let mut keys: Vec<&i32> = map.keys().collect();
    keys.sort();
    let mut vector: Vec<f32> = vec![];
    let mut i = 0;
    for k in keys {
        while i < *k {
            vector.push(0.0);
            i += 1;
        }
        vector.push(*map.get(&k).unwrap());
        i += 1;
    }
    while vector.len() > 0 && vector[vector.len() - 1] == 0.0 { vector.pop(); }
    vector
}

fn simplify_equations(left_eq: HashMap<i32, f32>, right_eq: HashMap<i32, f32>) -> HashMap<i32, f32> {
    let mut equation = left_eq.clone();
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
        return;
    }
    let poly = Poly::new(args.first().unwrap());
    if poly.is_err() {
        println!("Error parsing the polynomial equation");
        return;
    }
    let poly = poly.unwrap();
    println!("{:?}", poly.coefficients);
    println!("{}", poly.get_degree());
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
