fn is_leap_year(year: &str) -> bool {
    let year = year.parse::<i32>().unwrap();
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn totdays(date: &str) -> i32 {
    let parts: Vec<&str> = date.split('-').collect();
    let (year, month, day) = (parts[0], parts[1], parts[2]);
    let monthdays = if is_leap_year(year) {
        [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut totdays = 0;
    for i in 1..month.parse::<i32>().unwrap() {
        totdays += monthdays[i as usize];
    }
    totdays += day.parse::<i32>().unwrap();
    if year == "2026" {
        totdays += 365;
    }
    totdays
}

fn rest_days(date: &str) -> i32 {
    let year = date.split('-').collect::<Vec<&str>>()[0]
        .parse::<i32>()
        .unwrap();
    if is_leap_year(&year.to_string()) {
        366 - totdays(date)
    } else {
        365 - totdays(date)
    }
}

fn which_week(date: &str) -> i32 {
    let tot = totdays(date);
    let mut week = (tot + 2) / 7 + 1;
    if (tot + 2) % 7 == 0 {
        week -= 1;
    }
    if nxtday(date).split('-').collect::<Vec<&str>>()[1] == "01"
        && nxtday(date).split('-').collect::<Vec<&str>>()[2] == "01"
    {
        week = 1;
    }
    week
}

fn weekday(date: &str) -> i32 {
    // 周几
    let w: Vec<i32> = vec![2, 3, 4, 5, 6, 7, 1];
    return w[(totdays(date) % 7) as usize];
}

fn nxtday(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    let (year, month, day) = (
        parts[0].parse::<i32>().unwrap(),
        parts[1].parse::<i32>().unwrap(),
        parts[2].parse::<i32>().unwrap(),
    );
    let mday = if is_leap_year(&year.to_string()) {
        [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let ret = if day + 1 < mday[month as usize] {
        format!("{:04}-{:02}-{:02}", year, month, day + 1)
    } else if month + 1 < 13 {
        format!("{:04}-{:02}-{:02}", year, month + 1, 1)
    } else {
        format!("{:04}-01-01", year + 1)
    };

    ret
}

fn check_aday(date: &str) -> bool {
    let a_holiday: std::collections::HashSet<&str> = [
        "2025-01-01",
        "2025-01-26",
        "2025-01-28",
        "2025-01-29",
        "2025-01-30",
        "2025-01-31",
        "2025-02-01",
        "2025-02-02",
        "2025-02-03",
        "2025-02-04",
        "2025-02-08",
        "2025-04-04",
        "2025-04-05",
        "2025-04-27",
        "2025-05-01",
        "2025-05-02",
        "2025-05-03",
        "2025-05-04",
        "2025-05-05",
        "2025-05-31",
        "2025-06-01",
        "2025-06-02",
        "2025-10-01",
        "2025-10-02",
        "2025-10-03",
        "2025-10-04",
        "2025-10-05",
        "2025-10-06",
        "2025-10-07",
        "2025-10-08",
        "2025-09-28",
        "2025-10-11",
        "2026-01-01",
    ]
    .iter()
    .cloned()
    .collect();

    !(weekday(date) == 7 || weekday(date) == 6 || a_holiday.contains(date))
}

fn calc_next_aday(date: &str) -> i32 {
    if check_aday(&nxtday(date)) {
        // 如果下一天是开市日
        return 0;
    } else {
        let mut d: String = nxtday(date).to_string();
        while !check_aday(&d) {
            d = nxtday(&d);
        }
        return totdays(&d) - totdays(date) - 1;
    }
}

fn calc_spring_festival(date: &str) -> i32 {
    if date == "2025-01-29" || date == "2026-02-17" {
        return 0;
    }
    if totdays(date) < totdays("2025-01-29") {
        return totdays("2025-01-29") - totdays(date);
    } else {
        return rest_days(date) + totdays("2026-02-17") - 365;
    }
}

pub fn time_info(time: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    result.push(which_week(time).to_string());
    result.push(weekday(time).to_string());
    result.push(totdays(time).to_string());
    result.push(rest_days(time).to_string());
    result.push(calc_spring_festival(time).to_string());
    result.push(calc_next_aday(time).to_string());

    let res: String = format!(
        "{},{},{},{},{},{}",
        result[0], result[1], result[2], result[3], result[4], result[5]
    );
    // println!("{}", res);
    res.to_string()
}
