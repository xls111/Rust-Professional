pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    if threshold < 1 {
        return 0;
    }
    let mut sum = 1;
    let mut a = 1;
    let mut b = 1;
    while b < threshold {
        if b % 2 == 1 {
            sum += b;
        }
        let c = a + b;
        a = b;
        b = c;
    }
    sum
}
