use rand::Rng;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

#[derive(Debug, Clone)]
pub struct PolynomialRing {
    pub coefficients: Vec<Zq>,
}

impl PolynomialRing {
    pub const DEGREE_BOUND: usize = 64;

    pub fn new(coefficients: Vec<Zq>) -> Self {
        // Ensure coefficients are in Zq and degree is less than 64
        assert!(
            coefficients.len() <= Self::DEGREE_BOUND,
            "Polynomial degree must be less than 64"
        );
        PolynomialRing {
            coefficients: coefficients.clone(),
        }
    }
    // Multiply two polynomials in Rq = Zq[X]/(X^64 + 1)
    fn multiply_by_polynomial_ring(&self, other: &PolynomialRing) -> PolynomialRing {
        // Initialize a vector to hold the intermediate multiplication result
        let mut result_coefficients =
            vec![Zq::new(0); self.coefficients.len() + other.coefficients.len() - 1];
        for (i, &coeff1) in self.coefficients.iter().enumerate() {
            for (j, &coeff2) in other.coefficients.iter().enumerate() {
                result_coefficients[i + j] += coeff1 * coeff2;
            }
        }

        // Reduce modulo X^64 + 1
        if result_coefficients.len() > Self::DEGREE_BOUND {
            let modulus_minus_one = Zq::from(Zq::modulus() - 1);
            let (front, back) = result_coefficients.split_at_mut(Self::DEGREE_BOUND);
            for (i, &overflow) in back.iter().enumerate() {
                front[i] += overflow * modulus_minus_one;
            }
            result_coefficients.truncate(Self::DEGREE_BOUND);
            result_coefficients.truncate(Self::DEGREE_BOUND);
        }
        PolynomialRing {
            coefficients: result_coefficients,
        }
    }

    fn add_polynomial_ring(&self, other: &PolynomialRing) -> PolynomialRing {
        let len_a = self.coefficients.len();
        let len_b = other.coefficients.len();
        let max_len = std::cmp::max(len_a, len_b);
        let mut result_coefficients = Vec::with_capacity(max_len);

        result_coefficients.extend(
            self.coefficients
                .iter()
                .zip(other.coefficients.iter())
                .map(|(&a, &b)| a + b),
        );

        match len_a.cmp(&len_b) {
            std::cmp::Ordering::Greater => {
                result_coefficients.extend_from_slice(&self.coefficients[len_b..]);
            }
            std::cmp::Ordering::Less => {
                result_coefficients.extend_from_slice(&other.coefficients[len_a..]);
            }
            std::cmp::Ordering::Equal => {
                // Do nothing
            }
        }
        PolynomialRing {
            coefficients: result_coefficients,
        }
    }
}

impl Mul for PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: PolynomialRing) -> PolynomialRing {
        self.multiply_by_polynomial_ring(&other)
    }
}

impl Mul<&PolynomialRing> for PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: &PolynomialRing) -> PolynomialRing {
        self.multiply_by_polynomial_ring(other)
    }
}

impl Mul<PolynomialRing> for &PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: PolynomialRing) -> PolynomialRing {
        self.multiply_by_polynomial_ring(&other)
    }
}

impl Mul<&PolynomialRing> for &PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: &PolynomialRing) -> PolynomialRing {
        self.multiply_by_polynomial_ring(other)
    }
}

impl Mul<Zq> for PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: Zq) -> PolynomialRing {
        let new_coefficients = self.coefficients.iter().map(|c| *c * other).collect();
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl Mul<Zq> for &PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: Zq) -> PolynomialRing {
        let new_coefficients = self.coefficients.iter().map(|c| *c * other).collect();
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl Div<Zq> for PolynomialRing {
    type Output = PolynomialRing;

    fn div(self, other: Zq) -> PolynomialRing {
        let new_coefficients = self
            .coefficients
            .iter()
            .map(|c| Zq::new(c.value() / other.value()))
            .collect();
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl Add for PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: PolynomialRing) -> PolynomialRing {
        self.add_polynomial_ring(&other)
    }
}

impl Add<&PolynomialRing> for PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: &PolynomialRing) -> PolynomialRing {
        self.add_polynomial_ring(other)
    }
}

impl Add<PolynomialRing> for &PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: PolynomialRing) -> PolynomialRing {
        self.add_polynomial_ring(&other)
    }
}

impl Add<&PolynomialRing> for &PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: &PolynomialRing) -> PolynomialRing {
        self.add_polynomial_ring(other)
    }
}

impl Add<Zq> for PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: Zq) -> PolynomialRing {
        let mut new_coefficients = self.coefficients.clone();
        if let Some(first) = new_coefficients.get_mut(0) {
            *first += other;
        } else {
            new_coefficients.push(other);
        }
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl Add<&Zq> for PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: &Zq) -> PolynomialRing {
        let mut new_coefficients = self.coefficients.clone();
        if let Some(first) = new_coefficients.get_mut(0) {
            *first += *other;
        } else {
            new_coefficients.push(*other);
        }
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl Add<Zq> for &PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: Zq) -> PolynomialRing {
        let mut new_coefficients = self.coefficients.clone();
        if let Some(first) = new_coefficients.get_mut(0) {
            *first += other;
        } else {
            new_coefficients.push(other);
        }
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl Add<&Zq> for &PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: &Zq) -> PolynomialRing {
        let mut new_coefficients = self.coefficients.clone();
        if let Some(first) = new_coefficients.get_mut(0) {
            *first += *other;
        } else {
            new_coefficients.push(*other);
        }
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl PartialEq for PolynomialRing {
    fn eq(&self, other: &Self) -> bool {
        // Compare coefficients, ignoring trailing zeros
        let self_coeffs = self
            .coefficients
            .iter()
            .rev()
            .skip_while(|&&x| x == Zq::from(0))
            .collect::<Vec<_>>();
        let other_coeffs = other
            .coefficients
            .iter()
            .rev()
            .skip_while(|&&x| x == Zq::from(0))
            .collect::<Vec<_>>();

        self_coeffs == other_coeffs
    }
}

// Let q be a modulus, and let Zq be the ring of integers mod q
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zq {
    pub value: usize,
}

impl Zq {
    pub const Q: usize = 2usize.pow(32);

    pub fn modulus() -> usize {
        Self::Q
    }
    pub fn new(value: usize) -> Self {
        Zq {
            value: value % Self::Q,
        }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn pow(&self, other: usize) -> Self {
        Zq::new(self.value.pow(other as u32))
    }
}

impl PartialOrd for Zq {
    fn partial_cmp(&self, other: &Zq) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Display for Zq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Rem for Zq {
    type Output = Zq;

    fn rem(self, other: Zq) -> Zq {
        Zq::new(self.value % other.value)
    }
}

impl Div for Zq {
    type Output = Zq;

    fn div(self, other: Zq) -> Zq {
        Zq::new(self.value() / other.value())
    }
}

impl Add for Zq {
    type Output = Zq;

    fn add(self, other: Zq) -> Zq {
        Zq::new(self.value + other.value)
    }
}

impl AddAssign for Zq {
    fn add_assign(&mut self, other: Zq) {
        self.value = (self.value + other.value) % Self::Q;
    }
}

impl Sub for Zq {
    type Output = Zq;

    fn sub(self, other: Zq) -> Zq {
        Zq::new((self.value + Self::Q) - other.value)
    }
}

impl Mul for Zq {
    type Output = Zq;

    fn mul(self, other: Zq) -> Zq {
        Zq::new(self.value * other.value)
    }
}

impl From<usize> for Zq {
    fn from(value: usize) -> Self {
        Zq::new(value)
    }
}

impl Sum for Zq {
    fn sum<I: Iterator<Item = Zq>>(iter: I) -> Self {
        iter.fold(Zq::new(0), |acc, x| acc + x)
    }
}

#[derive(Debug)]
pub struct RqMatrix {
    pub values: Vec<Vec<PolynomialRing>>, // matrix of PolynomialRing values
}

impl RqMatrix {
    pub fn new(kappa: Zq, size_n: Zq) -> Self {
        let size_kappa_usize: usize = kappa.value();
        let size_n_usize: usize = size_n.value();
        let mut rng = rand::thread_rng();
        let values = (0..size_kappa_usize)
            .map(|_| {
                (0..size_n_usize)
                    .map(|_| PolynomialRing {
                        coefficients: (0..size_n_usize)
                            .map(|_| Zq::from(rng.gen_range(1..10)))
                            .collect(),
                    })
                    .collect()
            })
            .collect();
        RqMatrix { values }
    }
}
