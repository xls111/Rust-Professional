
#[derive(Debug)]
struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl <T> Stack<T> {
    fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn size(&self) -> usize {
        self.top
    }
    
}




pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7','8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

    let mut rem_stack = Stack::new();
    let num = &num_str[0..num_str.find('(').unwrap()];
    let base = num_str[num_str.find('(').unwrap() + 1..num_str.find(')').unwrap()].parse::<u32>().unwrap();

    let count = num.chars().count();
    let mut number = 0;
    for i in 0..count {
        let digit = num.chars().nth(i).unwrap();
        let mut value = 0;
        for j in 0..digits.len() {
            if digit == digits[j] {
                value = j as u32;
                break;
            }
        }
        number += value * base.pow((count - i - 1) as u32);

    }
    
    while number > 0 {
        let rem = number % to_base;
        rem_stack.push(rem);
        number /= to_base;
    }

    let mut new_num_str = String::new();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap();
        new_num_str.push_str(&digits[rem as usize].to_string());
    }
    new_num_str
}
