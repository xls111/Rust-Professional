

#[derive(Debug, Clone, Copy)]
struct Date {
    year: i32,
    month: i32,
}

enum WorkerType {
    Man,
    Woman55,
    Woman50,
}

impl Date {

    fn new(year: i32, month: i32) -> Self {
        Date { year, month }
    }

    fn from_str(date_str: &str) -> Option<Self> {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 2 {
            return None;
        }
        
        let year = parts[0].parse().ok()?;
        let month = parts[1].parse().ok()?;
        
        if month < 1 || month > 12 {
            return None;
        }
        
        Some(Date { year, month })
    }
    
    fn add_months(&self, months: i32) -> Self {
        let total_months = self.year * 12 + self.month - 1 + months;
        let year = total_months / 12;
        let month = total_months % 12 + 1;
        
        Date { year, month }
    }
    
    fn months_between(&self, other: &Date) -> i32 {
        (other.year * 12 + other.month) - (self.year * 12 + self.month)
    }
    
    fn format(&self) -> String {
        format!("{}-{:02}", self.year, self.month)
    }
}

fn calculate_retirement_params(birth_date: &Date, worker_type: WorkerType) -> (Date, f32, i32) {
    let base_retirement_age = match worker_type {
        WorkerType::Man => 60,
        WorkerType::Woman55 => 55,
        WorkerType::Woman50 => 50,
        _ => 60,
    };
    
    // 计算延迟月数
    let delay_months = match worker_type {
        WorkerType::Man => {
            if birth_date.year >= 1965 {
                let org_retire_year = birth_date.year + base_retirement_age;
                (Date::new(2025,1).months_between(&Date::new(org_retire_year,birth_date.month)) / 4 + 1 ).min(36) 
            } else {
                0
            }
        },
        WorkerType::Woman55 => {
            if birth_date.year >= 1970 {
                let org_retire_year = birth_date.year + base_retirement_age;
                (Date::new(2025,1).months_between(&Date::new(org_retire_year,birth_date.month)) / 4 + 1 ).min(36) 
            } else {
                0
            }
        },
        WorkerType::Woman50 => {
            if birth_date.year >= 1975 {
                let org_retire_year = birth_date.year + base_retirement_age;
                (Date::new(2025,1).months_between(&Date::new(org_retire_year,birth_date.month)) / 2 + 1 ).min(60) 
            } else {
                0
            }
        }
    };

    let retirement_date = birth_date.add_months(base_retirement_age * 12 + delay_months);
    let actual_age = base_retirement_age as f32 + (delay_months as f32 / 12.0);

    (retirement_date, actual_age, delay_months)
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let birth_date = Date::from_str(time).unwrap();

    let worker_type = match tp {
        "男职工" => WorkerType::Man,
        "原法定退休年龄55周岁女职工" => WorkerType::Woman55,
        "原法定退休年龄50周岁女职工" => WorkerType::Woman50,
        _ => panic!("Invalid worker type"),
    };

    let (retirement_date, actual_age, remaining_months) = calculate_retirement_params(&birth_date, worker_type);
    
    if actual_age == actual_age.floor() {
        return format!("{},{},{}", 
            retirement_date.format(),
            actual_age as i32,
            remaining_months);
    }

    format!("{},{:.2},{}", 
        retirement_date.format(),
        actual_age,
        remaining_months)
}
