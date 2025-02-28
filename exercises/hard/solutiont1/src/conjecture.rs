pub fn goldbach_conjecture() -> String {
    let mut odd_numbers = Vec::new();
    
    // 从3开始检查奇数
    let mut n = 3;
    while odd_numbers.len() < 2 {
        if !can_be_represented(n) && !is_prime(n){
            odd_numbers.push(n);
        }
        n += 2;
    }
    
    format!("{},{}", odd_numbers[0], odd_numbers[1])
}

// 判断一个数是否可以表示为一个素数和一个平方数的两倍之和
fn can_be_represented(n: i32) -> bool {
    // 对于每个小于n的素数
    for p in (2..n).filter(|&x| is_prime(x)) {
        // 检查 n-p 是否是某个数的平方的两倍
        let remaining = n - p;
        if remaining <= 0 {
            continue;
        }
        if remaining % 2 == 0 {
            let half = remaining / 2;
            let sqrt = (half as f64).sqrt() as i32;
            if sqrt * sqrt == half {
                return true;
            }
        }
    }
    false
}

// 判断一个数是否为素数
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt = (n as f64).sqrt() as i32;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
