use num_bigint::{BigInt, BigUint};

//alpha^x mod p
//Output  = number*exp mod p
pub fn exponentiate(number : &BigUint, exponent : &BigUint, modulus: &BigUint) -> BigUint {
    number.modpow(exponent, modulus)
}

//output = s= k - c * x mod q
pub fn solve (K : &BigUint, C : &BigUint, X: &BigUint, q: &BigUint) -> BigUint {
        if *K >= C*X {
            (K-C*X).modpow(&BigUint::from(1u32), q);
        }
    q- (C * X - K).modpow(&BigUint::from(1u32), q)
}

//cond1 = r1 = alpha^s * y1^c
//cond2 = r2 = beta^s * y2*c
pub fn verify (r1 : &BigUint, r2 : &BigUint, y1: &BigUint, y2: &BigUint, alpha: &BigUint, beta: &BigUint, c: &BigUint, s: &BigUint, modulus : &BigUint) -> bool {
    let cond1 = *r1 == alpha.modpow(s,modulus) * y1.modpow(c,modulus);
    let cond2 = *r2 == beta.modpow(s,modulus) * y2.modpow(c,modulus);

    cond1 && cond2
}