use num_bigint::{RandBigInt, BigUint};
//alpha^x mod p
//Output  = number*exp mod p
pub fn exponentiate(number : &BigUint, exponent : &BigUint, modulus: &BigUint) -> BigUint {
    number.modpow(exponent, modulus)
}

//output = s= k - c * x mod q
pub fn solve (k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
        if *k >= c * x {
            (k - c * x).modpow(&BigUint::from(1u32), q);
        }
    q- (c * x - k).modpow(&BigUint::from(1u32), q)
}

//cond1 = r1 = alpha^s * y1^c
//cond2 = r2 = beta^s * y2*c
pub fn verify (r1 : &BigUint, r2 : &BigUint, y1: &BigUint, y2: &BigUint, alpha: &BigUint, beta: &BigUint, c: &BigUint, s: &BigUint, modulus : &BigUint) -> bool {
    let cond1 = *r1 == (alpha.modpow(s,modulus) * y1.modpow(c,modulus)).modpow(&BigUint::from(1u32), &modulus);
    let cond2 = *r2 == (beta.modpow(s,modulus) * y2.modpow(c,modulus)).modpow(&BigUint::from(1u32), &modulus);

    cond1 && cond2
}

pub fn generate_random_below(bound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    rng.gen_biguint_below(bound)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
    let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let modulus = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let x = BigUint::from(6u32);
        let k = generate_random_below(&modulus);

        let c=generate_random_below(&modulus);

        let y1 = exponentiate(&alpha,&x,&modulus);
        let y2 = exponentiate(&beta,&x,&modulus);

        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = exponentiate(&alpha,&k,&modulus);
        let r2 = exponentiate(&beta,&k,&modulus);

        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = solve(&k,&c, &x, &q);
        assert_eq!(s, BigUint::from(5u32));

        let result = verify(&r1,&r2,&y1,&y2,&alpha, &beta, &c, &s, &modulus);
        assert!(result);

        //Compute fake secret
        //s = k -c * x mod q
        let x_fake = BigUint::from(7u32);
        let s_fake = solve(&k,&c, &x_fake, &q);
        assert_eq!(s, BigUint::from(5u32));

        let result = verify(&r1,&r2,&y1,&y2,&alpha, &beta, &c, &s_fake, &modulus);
        assert!(!result);
    }
}