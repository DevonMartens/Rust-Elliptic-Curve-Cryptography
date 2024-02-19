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
        fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
            let r = c + d;
            r % p // Perform modular addition
        }
    fn sub(c: BigUint, d: BigUint) -> BigUint {
        //(c - d) % self.p
        todo!("Implement the finite field operations")
    }
    fn mul(c: BigUint, d: BigUint) -> BigUint {
        // (c * d) % self.p
        todo!("Implement the finite field operations")
    }
    fn div(c: BigUint, d: BigUint) -> BigUint {
        todo!("Implement the inverse addition function")
    }
    fn inverse_addition(c: BigUint) -> BigUint {
        todo!("Implement the inverse addition function")
    }
    fn inverse_multiplication(c: BigUint) -> BigUint {
    todo!("Implement the finite field operations")
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
}