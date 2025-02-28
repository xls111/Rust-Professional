use std::cmp::Ordering;

// 节假日数据集 (示例数据)
static HOLIDAYS: [&str;29] = [
    "2025-01-01", // 元旦
    "2025-01-28","2025-01-29","2025-01-30","2025-01-31","2025-02-01","2025-02-02","2025-02-03","2025-02-04", // 春节假(示例)
    "2025-04-04", "2025-04-05", "2025-04-06", // 清明
    "2025-05-01", "2025-05-02", "2025-05-03", "2025-05-04","2025-05-05", // 劳动节
    "2025-05-31", "2025-06-01", "2025-06-02", // 端午节
    "2025-10-01", "2025-10-02", "2025-10-03", "2025-10-04", "2025-10-05", "2025-10-06", "2025-10-07","2025-10-08", // 国庆节
    "2026-01-01"
    ];

// 调休工作日数据集 (周末上班)
static WORKDAY_WEEKENDS: [&str; 5] = [
    "2025-01-26", "2025-02-08",  // 春节调休
    "2025-04-27", 
    "2025-09-28",
    "2025-10-11",
];

#[derive(Debug, Clone, Copy,PartialEq)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_to_date(day: u32,year: u32) -> Result<Date, &'static str> {
    let mut MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day = day;

    if is_leap_year(year) {
        MONTH_DAYS[1] = 29;
    }

    let mut month = 0;
    while month < 12 && day > MONTH_DAYS[month] {
        day -= MONTH_DAYS[month];
        month += 1;
    }

    if month >= 12 || day == 0 {
        return Err("Invalid date calculation");
    }

    Ok(Date::new(year, (month + 1).try_into().unwrap(), day))
}

/// 计算指定公历年份的春节日期
fn calculate_spring_festival(year: u32) -> Result<Date, &'static str> {
    match year {
        2020 => Ok(Date::new(2020, 1, 25)),
        2021 => Ok(Date::new(2021, 2, 12)),
        2022 => Ok(Date::new(2022, 2, 1)),
        2023 => Ok(Date::new(2023, 1, 22)),
        2024 => Ok(Date::new(2024, 2, 10)),
        2025 => Ok(Date::new(2025, 1, 29)),
        2026 => Ok(Date::new(2026, 2, 17)),
        2027 => Ok(Date::new(2027, 2, 6)),
        2028 => Ok(Date::new(2028, 1, 26)),
        2029 => Ok(Date::new(2029, 2, 13)),
        _ => Err("Invalid year"),
    }
}

impl Date {

    fn new(year: u32, month: u32, day: u32) -> Self {
        Date { year, month, day }
    }

    fn from_str(date_str: &str) -> Option<Self> {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return None;
        }

        let mut days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        
        let year = parts[0].parse().ok()?;
        let month = parts[1].parse().ok()?;
        let day = parts[2].parse().ok()?;
        
        if is_leap_year(year){
            days[1] = 29;
        }

        if month < 1 || month > 12 {
            return None;
        }

        if days[month as usize - 1] < day || day < 1 {
            return None;
        }
        
        Some(Date { year, month, day })
    }
    
    fn add_months(&self, months: u32) -> Self {
        let total_months = self.year * 12 + self.month - 1 + months;
        let year = total_months / 12;
        let month = total_months % 12 + 1;
        let day = self.day;
        
        Date { year, month, day }
    }
    
    fn add_days(&self, days:u32) -> Self {
        let mut month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut year = self.year;
        let mut month = self.month;
        let mut day = self.day + days;
        if is_leap_year(year){
            month_days[1] = 29;
        }

        while day > month_days[month as usize - 1] {
            day -= month_days[month as usize - 1];
            month += 1;
            if month > 12 {
                month = 1;
                year += 1;
            }
        }

        
        Date { year, month, day }
    }

    fn sub_days(&self, days:u32) -> Self {
        let mut month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut year = self.year;
        let mut month = self.month;
        let mut day = self.day as i32 - days as i32;
        if is_leap_year(year){
            month_days[1] = 29;
        }
        
        while day < 1 {
            month -= 1;
            if month < 1 {
                month = 12;
                year -= 1;
            }
            day += month_days[month as usize - 1];
        }

        Date { year, month, day:day.try_into().unwrap() }
    }

    fn day_of_week(&self) -> u32 {
        let mut y = self.year;
        let mut m = self.month;
        if m == 1 || m == 2 {
            y -= 1;
            m += 12;
        }

        let h = (self.day + 2 * m + (3 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400 + 1) % 7;

        if h == 0 {
            7
        } else {
            h
        }
    }

    fn offset_days_from_year_start(&self) -> u32 {
        let mut days = self.day;
        let mut month = self.month;
        while month > 1 {
            month -= 1;
            days += match month {
                1 => 31,
                2 => if is_leap_year(self.year) { 29 } else { 28 },
                3 => 31,
                4 => 30,
                5 => 31,
                6 => 30,
                7 => 31,
                8 => 31,
                9 => 30,
                10 => 31,
                11 => 30,
                12 => 31,
                _ => 0,
            };
        }
        days
    }

    fn is_boundary_week(&self) -> bool {
        let day_in_week = self.day_of_week();
        let start_of_week = self.sub_days(day_in_week - 1);
        let end_of_week = start_of_week.add_days(6);
        if start_of_week.year != end_of_week.year {
            return true;
        }
        false
    }

    fn week_of_year(&self) -> u32 {
        // 既要判断这一周周一是否在这一年，也要判断下一年第一天是否在这一周，它是不是周四
        let day_in_week = self.day_of_week();
        let start_of_week = self.sub_days(day_in_week - 1);
        let year_start = Date::new(self.year, 1, 1);
        if self.is_boundary_week() && self.month == 12 {
            let next_year_start = Date::new(self.year + 1, 1, 1);
            if next_year_start.day_of_week() <= 4 {
                return 1;
            }
        }

        let a = if year_start.day_of_week() <= 4 && start_of_week.year != self.year {
            1
        } else {
            0
        };

        let b = if start_of_week.year == self.year {
            1
        } else {
            0
        };
       
        let days =  if start_of_week.year != self.year {
            0
        } else {
            self.offset_days_from_year_start()
        };

        let week = days / 7 + a + b;
        
        week
    }

    fn remain_days_in_year(&self) -> u32 {
        let mut days = 365;
        if is_leap_year(self.year) {
            days = 366;
        }
        days - self.offset_days_from_year_start()
    }

    fn remain_days_for_spring_festival(&self) -> u32 {
        let mut spring_festival = calculate_spring_festival(self.year).unwrap();
        if self > &spring_festival {
            spring_festival = calculate_spring_festival(self.year + 1).unwrap();
        }

        self.days_between(&spring_festival)
    }

    fn is_weekend(&self) -> bool {
        let day_of_week = self.day_of_week();
        day_of_week == 6 || day_of_week == 7
    }

    // 判断是否是工作日,节假日、周末不算工作日
    fn is_workday(&self) -> bool {
        // First check if it's weekend (Saturday = 6 or Sunday = 7)
        let date_str:&str = &self.format();
        

        // Then check if it's a holiday
        if HOLIDAYS.contains(&date_str) {
            return false;
        }

        // Finally check if it's a workday weekend
        if WORKDAY_WEEKENDS.contains(&date_str) {
            return true;
        }

        !self.is_weekend()
    }

    fn is_tradeday(&self) -> bool {
        if self.is_workday() {
            if self.day_of_week() < 6 {
                return true;
            }
        }
        false
    }

    fn remain_days_for_A_stock_opening(&self) -> u32 {
        
        let mut date = self.clone();

        if self.is_tradeday() {
            date = date.add_days(1);
        }

        while !date.is_tradeday() {
            date = date.add_days(1);
        }
        
        self.days_between(&date) - 1
    }

    // 计算两个日期之间的天数,但仅限于比当前日期靠后的日期
    fn days_between(&self, other: &Date) -> u32 {
        let mut days = 0;
        let start = self.offset_days_from_year_start();
        let end = other.offset_days_from_year_start();
        if self.year == other.year {
            days = end - start;
        } else {
            days = self.remain_days_in_year() + other.offset_days_from_year_start();
            if other.year - self.year > 1 {
                for i in 1..other.year - self.year {
                    days += if is_leap_year(self.year + i) { 366 } else { 365 };
                }
            }
        }

        days
    }


    fn format(&self) -> String {
        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.year < other.year {
            return Some(Ordering::Less);
        } else if self.year > other.year {
            return Some(Ordering::Greater);
        }

        if self.month < other.month {
            return Some(Ordering::Less);
        } else if self.month > other.month {
            return Some(Ordering::Greater);
        }

        if self.day < other.day {
            return Some(Ordering::Less);
        } else if self.day > other.day {
            return Some(Ordering::Greater);
        }

        Some(Ordering::Equal)
    }
    
}


pub fn time_info(time: &str) -> String {
    let date = Date::from_str(time).unwrap();
    let week_of_year = date.week_of_year();
    let day_of_week = date.day_of_week();
    let offset_days_from_year_start = date.offset_days_from_year_start();
    let remain_days_in_year = date.remain_days_in_year();
    let remain_days_for_spring_festival = date.remain_days_for_spring_festival();
    let remain_days_for_a_stock_opening = date.remain_days_for_A_stock_opening();

    format!("{},{},{},{},{},{}", week_of_year, day_of_week, offset_days_from_year_start, remain_days_in_year,remain_days_for_spring_festival,remain_days_for_a_stock_opening)
}
