use num_bigint::BigUint;

pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {   // n^exp mod p
    n.modpow(exponent, modulus)
}

pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {   //s = k-cx mod q
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q);
    }
    return (q + k - c * x).modpow(&BigUint::from(1u32), q);
}

pub fn verify(r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, alpha: &BigUint, beta: &BigUint, c: &BigUint, s: &BigUint, p: &BigUint) -> bool {
    let cond1 = *r1 == alpha.modpow(s, p) * y1.modpow(c, p);
    let cond2 = *r2 == alpha.modpow(s, p) * y2.modpow(c, p);
    cond1 && cond2
}