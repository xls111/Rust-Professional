pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑,保留四位小数
    
    let mut a ;
    let b = 365;
    let mut c = 1.0;
    for i in 0..n {
        a = 365 - i;
        c *= a as f64 / b as f64;
    }

    ((1.0 - c) * 10000.0).round() / 10000.0
}
