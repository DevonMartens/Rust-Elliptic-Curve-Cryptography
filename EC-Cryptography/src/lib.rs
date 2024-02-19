// big integer library
use num_bigint::BigUint;




// Point on the elliptic curve
enum Point {
    Coordinate(BigUint, BigUint),
    Identity,
}
struct EllipticCurve {
    // y^2 = x^3 + ax + b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

// implement addition 
impl EllipticCurve {
  //  passing points by reference
    fn add(&self, c: &Point, d: &Point) -> Point {
        assert!(
            self.is_on_curve(c),
            "The first point is not on the curve"
        );
        assert!(
            self.is_on_curve(d),
            "The second point is not on the curve"
        );
        match (c,d) => {
            (Point::Identity, _) => d,
            (_, Point::Identity) => c,
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                // s= (y2-y1)/(x2-x1) mod p
                // x3 = s^2 - x1 - x2 mod p
                // y3 = s(x1-x3) - y1  mod p
                let s: () =
                if x1 == x2 && y1 != y2 {
                    return Point::Identity;
                }
                if x1 == x2 && y1 == y2 {
                    return self.double(c);
                }
                let lambda = if x1 == x2 {
                    (BigUint::from(3u32) * x1.pow(BigUint::from(2u32)) + &self.a) * FiniteField::inverse_multiplication(&BigUint::from(2u32) * y1, &self.p)
                } else {
                    (y2 - y1) * FiniteField::inverse_multiplication(&(x2 - x1), &self.p)
                };
                let x3 = lambda.pow(BigUint::from(2u32)) - x1 - x2;
                let y3 = lambda * (x1 - x3) - y1;
                return Point::Coordinate(x3, y3);
            }
        }

    }

    fn double(c: &Point){
        todo!("Implement the double function")
    }
    fn scalar_mul(c: &Point, n: BigUint) -> Point {
        todo!("Implement the multiply function")
    }
    fn is_on_curve(c: &Point) -> bool {
        // y^2 = x^3 + ax + b
        let y2 = match c {
           Point::Coordinate(x, y) => {
            let y2 = y.modpow(&BigUint::from(2u32), &this.p)
            let x3 = x.modpow(&BigUint::from(3u32), &this.p)
            let ax = &self.a * x
            y2 == x3 + ax + &self.b;
           }
            Point::Identity => return false,
        };
        todo!("Implement the is_on_curve function")
    }
}

struct FiniteField {}

impl FiniteField {
        // Correctly defined as a static method
    pub fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let r = c + d;
        return r % p; // Perform modular addition
    }
    pub fn mult(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let r: BigUint = c * d;
        return r.modpow(&BigUint::from(1u32), p) // Perform modular multiplication
      
    }
    pub fn inv_addition(c: &BigUint, p: &BigUint) -> BigUint {
        assert!(
            c < p,
            "The first number should be less than the second number"
        );
        let r = p - c;
        return r % p;
        
    }
    pub fn inverse_addition(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv = FiniteField::inv_addition(d, p);
    }
    pub fn inverse_multiplication(c: &BigUint, p:&BigUint) -> BigUint {
        let r = c.modpow(&(p - BigUint::from(2u32)), p);
        return r;
}
    pub fn subtract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let r = c - d;
        return r % p;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(11u32);
        let r = FiniteField::add(&c, &d, &p);
        assert_eq!(r, BigUint::from(3u32));
    }
    #[test]
    fn test_mult() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(11u32);
        let r = FiniteField::mult(&c, &d, &p);
        assert_eq!(r, BigUint::from(7u32));
    }
    #[test]
    fn test_mult2() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(51u32);
        let r = FiniteField::mult(&c, &d, &p);
        assert_eq!(r, BigUint::from(40u32));
    }
    #[test]
    fn test_inv_addition() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(11u32);
        let r = FiniteField::inv_addition(&c, &p);
        assert_eq!(r, BigUint::from(7u32));
    }
    #[test]
    fn test_inv_addition2() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(51u32);
        let r = FiniteField::inv_addition(&c, &p);
        assert_eq!(r, BigUint::from(47u32));
    }
    #[test]
    #[should_panic]
    fn test_inv_addition3() {
        let c = BigUint::from(11u32);
        let p = BigUint::from(11u32);
        let r = FiniteField::inv_addition(&c, &p);
    }
    #[test]
    fn test_inverse_multiplication() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(11u32);
        let r = FiniteField::inverse_multiplication(&c, &p);
        assert_eq!(r, BigUint::from(3u32));
    }
    #[test]
    fn test_inverse_multiplication2() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(11u32);
        let r = FiniteField::inverse_multiplication(&c, &p);
        assert_eq!(r, BigUint::from(3u32));
    }
    #[test]

}