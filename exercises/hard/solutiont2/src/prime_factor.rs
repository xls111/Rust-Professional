// 快速乘法函数，避免大数溢出
fn mod_mul(mut a: u128, mut b: u128, m: u128) -> u128 {
    let mut result = 0;
    a %= m;
    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    result
}

// 使用快速乘法的快速幂
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp >>= 1;
    }
    result
}

// Miller-Rabin 素性测试
fn is_prime(n: u128) -> bool {
    if n <= 1 || n == 4 { return false; }
    if n <= 3 { return true; }

    let bases = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    let mut d = n - 1;
    let mut s = 0;

    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    for &a in bases.iter().take_while(|&&x| x < n) {
        if !witness(a, d, n, s) {
            return false;
        }
    }
    true
}

fn witness(mut a: u128, d: u128, n: u128, s: u32) -> bool {
    a = mod_pow(a, d, n);
    if a == 1 || a == n - 1 {
        return true;
    }
    
    for _ in 1..s {
        a = mod_mul(a, a, n);
        if a == n - 1 {
            return true;
        }
        if a == 1 {
            return false;
        }
    }
    false
}

pub fn find_max_prime_factor(number: u128) -> u128 {
    if number <= 1 {
        return number;
    }

    let mut n = number;
    let mut max_prime = 1;

    while n % 2 == 0 {
        max_prime = 2;
        n /= 2;
    }

    while n % 3 == 0 {
        max_prime = 3;
        n /= 3;
    }

    // If n is prime after removing 2 and 3 factors, return it
    if is_prime(n) {
        return n;
    }

    let mut i = 5;
    let mut increment = 2;

    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                max_prime = i;
                n /= i;
                if is_prime(n) {
                    return n;
                }
            }
        }
        i += increment;
        increment = 6 - increment;
    }

    if n > max_prime {
        n
    } else {
        max_prime
    }
}
