// big integer library
use num_bigint::BigUint;




// Point on the elliptic curve
struct Point {
    x: BigUint,
    y: BigUint,
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
    fn add(c: &Point, d: &Point) -> Point {
        todo!("Implement the add function");
    }

    fn double(c: &Point){
        todo!("Implement the double function")
    }
    fn scalar_mul(c: &Point, n: BigUint) -> Point {
        todo!("Implement the multiply function")
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
    pub fn inverse_multiplication(c: &BigUint, p:&BigUint) -> BigUint {
        let r = c.modpow(&(p - BigUint::from(2u32)), p);
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
}