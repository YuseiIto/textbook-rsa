fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }
    true
}

fn euler_phi(p: u64, q: u64) -> u64 {
    if !is_prime(p) || !is_prime(q) {
        panic!("p and q must be prime numbers");
    }

    (p - 1) * (q - 1)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn find_coprime(l: u64) -> u64 {
    for i in 2..l {
        if gcd(i, l) == 1 {
            return i;
        }
    }
    0
}

fn is_mod_inverse(s: u64, t: u64, n: u64) -> bool {
    (s * t) % n == 1
}

fn find_mod_inverse(e: u64, n: u64) -> u64 {
    (1..n).filter(|i| is_mod_inverse(*i, e, n)).next().unwrap()
}

fn fast_pow(base: u64, exp: u64, m: u64) -> u64 {
    let mut result = 1;
    let mut base = base;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % m;
        }
        exp = exp >> 1;
        base = (base * base) % m;
    }
    result
}

fn encrypt(msg: u64, e: u64, n: u64) -> u64 {
    fast_pow(msg, e, n) % n
}

fn decrypt(cipher: u64, d: u64, n: u64) -> u64 {
    fast_pow(cipher, d, n) % n
}

fn main() {
    let p = 17; // prime
    let q = 19; // prime
    let n = p * q;
    let phi = euler_phi(p, q);
    let e = find_coprime(phi);
    let d = find_mod_inverse(e, phi);

    let public_key = (e, n);
    let private_key = d;

    println!("Public key: {:?}", public_key);
    println!("Private key: {:?}", private_key);

    let msg = 128u64;

    println!("Message: {}", msg);

    let cipher = encrypt(msg, e, n);

    println!("Cipher: {}", cipher);

    let decrypted = decrypt(cipher, d, n);

    println!("Decrypted message: {}", decrypted);

    if decrypted != msg {
        println!("Decryption failed");
    }
}
