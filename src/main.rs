fn add_days_to_date(mut date: Date) {
    let mut days_left_in_month: i32;
    while date.days_left_to_add > 0 {
        days_left_in_month = date.days_in_month[date.month as usize] - date.day;
        if date.month == 2 && is_leap_year(date.year) {
            days_left_in_month += 1;
        }
        if date.days_left_to_add > days_left_in_month {
            date.days_left_to_add -= days_left_in_month + 1;
            date.day = 1;
            if date.month == 12 {
                date.month = 1;
                date.year += 1;
            } else {
                date.month += 1;
            }
        } else {
            date.day = date.day + date.days_left_to_add;
            date.days_left_to_add = 0;
        }
    }
    println!("{} {} {}", date.month, date.day, date.year);
}

fn is_leap_year(year: i32) -> bool {
    if year % 4 != 0 {
        false
    } else if year % 100 != 0 {
        true
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}

struct Date {
    month: i32,
    day: i32,
    year: i32,
    days_left_to_add: i32,
    days_in_month: Vec<i32>,
}

fn main() {
    let date1 = Date {
        month: 12,
        day: 31,
        year: 2019,
        days_left_to_add: 367,
        days_in_month: vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    };

    add_days_to_date(date1);
}
