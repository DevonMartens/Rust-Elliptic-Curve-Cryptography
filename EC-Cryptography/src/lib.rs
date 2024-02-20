// big integer library
use num_bigint::BigUint;


// Point on the elliptic curve
#[derive(PartialEq, Clone, Debug)]
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
        match (c,d) {
            (Point::Identity, _) => d.clone(),
            (_, Point::Identity) => c.clone(),
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                // s= (y2-y1)/(x2-x1) mod p
                // x3 = s^2 - x1 - x2 mod p
                // y3 = s(x1-x3) - y1  mod p
                let y2minusy1 = FiniteField::subtract(y2, y1, &self.p);
                let x2minusx1 = FiniteField::subtract(x2, x1, &self.p);
                let s = FiniteField::divide(&y2minusy1, &x2minusx1, &self.p);
                let s2 = s.modpow(&BigUint::from(2u32), &self.p); 
                let s2minusx1 = FiniteField::subtract(&s2, x1, &self.p);
                let x3 = FiniteField::subtract(&s2minusx1, x2, &self.p);
                let x1minusx3 = FiniteField::subtract(x1, &x3, &self.p);
                let sx1minusx3 = FiniteField::mult(&s, &x1minusx3, &self.p);
                let y3 = FiniteField::subtract(&sx1minusx3, &y1, &self.p);
                Point::Coordinate(x3 % &self.p, y3 % &self.p)
            }
        }
    }
    fn double(c: &Point){
        todo!("Implement the double function")

    }
    fn scalar_mul(&self, c: &Point, n: BigUint) -> Point {
        let mut r = Point::Identity;
        let mut m = c.clone();
        let mut i = n.clone();
        while i > BigUint::from(0u32) {
            if i.clone()  % BigUint::from(2u32) == BigUint::from(1u32) {
                r = self.add(&r, &m);
            }
            m = self.add(&m, &m);
            i = i % BigUint::from(2u32);
        }
        return r;
    }
    pub fn is_on_curve(&self, a: &Point) -> bool {
        match a {
            Point::Coordinate(x, y) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = FiniteField::mult(&self.a, x, &self.p);
                let x3plusax = FiniteField::add(&x3, &ax, &self.p);

                y2 == FiniteField::add(&x3plusax, &self.b, &self.p)
            }
            Point::Identity => true,
        }
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
        let r = FiniteField::add(c, &d_inv, p);
        return r;
    }
    pub fn inverse_multiplication(c: &BigUint, p:&BigUint) -> BigUint {
        let r = c.modpow(&(p - BigUint::from(2u32)), p);
        return r;
}
    pub fn subtract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv = FiniteField::inv_addition(d, p);
        let r = FiniteField::add(c, &d_inv, p);
        return r ;
    }
    pub fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv = FiniteField::inverse_multiplication(d, p);
        let r = FiniteField::add(c, &d_inv, p);
        return r;
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
        let _r = FiniteField::inv_addition(&c, &p);
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

    fn test_ec_point_addition() {
        let a = BigUint::from(2u32);
        let b = BigUint::from(2u32);
        let p = BigUint::from(17u32);
        let curve = EllipticCurve { a, b, p };
    
        // Points must be on the curve. Assuming these are valid for demonstration.
        let c = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
      //  let d = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
      let d = Point::Identity;
    
        // Expected result after addition. This needs to be a valid point on the curve after adding c and d.
        let pr = c.clone();
    
        let res = curve.add(&c, &d);
        assert_eq!(res, pr);
    }
}