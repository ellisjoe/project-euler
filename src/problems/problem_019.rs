#[test]
fn problem_019() {
    let result = DateIterator { date: Date::jan_1_1900() }
        .skip_while(|d| d.year != 1901)
        .take_while(|d| d.year != 2001)
        .filter(|d| d.is_sunday_on_the_first())
        .count();
    println!("{}", result);
}

struct DateIterator {
    date: Date,
}

impl DateIterator {
    fn new(date: Date) -> Self {
        Self { date }
    }
}

impl Iterator for DateIterator {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        self.date = self.date.incr();
        Some(self.date.clone())
    }
}

#[derive(Clone, Debug)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    day_of_week: DayOfWeek,
}

impl Date {
    fn jan_1_1900() -> Date {
        Date {
            year: 1900,
            month: 1,
            day: 1,
            day_of_week: DayOfWeek::Monday,
        }
    }

    fn incr(&self) -> Date {
        if self.month == 12 && self.day == 31 {
            return Date {
                year: self.year + 1,
                month: 1,
                day: 1,
                day_of_week: self.day_of_week.next(),
            };
        }

        let days_in_month = match self.month {
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => 31,
        };

        if self.day == days_in_month {
            Date {
                year: self.year,
                month: self.month + 1,
                day: 1,
                day_of_week: self.day_of_week.next(),
            }
        } else {
            Date {
                year: self.year,
                month: self.month,
                day: self.day + 1,
                day_of_week: self.day_of_week.next(),
            }
        }
    }

    fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && (!self.is_century() || self.year % 400 == 0)
    }

    fn is_century(&self) -> bool {
        self.year % 100 == 0
    }

    fn is_sunday_on_the_first(&self) -> bool {
        self.day_of_week == DayOfWeek::Sunday && self.day == 1
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl DayOfWeek {
    fn next(&self) -> DayOfWeek {
        match self {
            DayOfWeek::Sunday => DayOfWeek::Monday,
            DayOfWeek::Monday => DayOfWeek::Tuesday,
            DayOfWeek::Tuesday => DayOfWeek::Wednesday,
            DayOfWeek::Wednesday => DayOfWeek::Thursday,
            DayOfWeek::Thursday => DayOfWeek::Friday,
            DayOfWeek::Friday => DayOfWeek::Saturday,
            DayOfWeek::Saturday => DayOfWeek::Sunday,
        }
    }
}
