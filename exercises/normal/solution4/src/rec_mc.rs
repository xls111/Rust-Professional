pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    if amount <= 0 {
        return 0;
    }
    let mut coins = vec![1, 2, 5, 10, 20, 30, 50, 100].into_iter().filter(|x| *x <= amount).rev();
    let mut count = 0;
    let mut amount_ = amount;
    while let Some(coin) = coins.next() {

        if amount_ == 0 {
            break;
        }
        let a = amount_ / coin;
        if a > 0 {
            count += a;
            amount_ %= coin;
        }
       
    }
    count
}
