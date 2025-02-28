pub fn new_count_distinct(input_str: &str) -> usize {
    // 将input_str按逗号分割成字符串数组
    let strs = input_str.split(',').collect::<Vec<&str>>();
    let mut length = 0;
    let mut unique = Vec::new();
    for s in strs {
        if !unique.contains(&s) {
            unique.push(s);
            length += 1;
        }
    }
    length
}
