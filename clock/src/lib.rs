use core::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: 0,
            minutes: 0,
        }
        .add_hour(hours)
        .add_minutes(minutes)
    }

    pub fn add_hour(&self, hours: i32) -> Self {
        let mut h = self.hours + hours;
        while h > 23 {
            h -= 24;
        }
        while h < 0 {
            h += 24;
        }
        Clock {
            hours: h,
            minutes: self.minutes,
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes = self.minutes + minutes;
        if self.minutes >= 60 {
            while self.minutes >= 60 {
                self.minutes -= 60;
                self.hours += 1;
            }

            while self.hours > 23 {
                self.hours -= 24;
            }
        } else {
            while self.minutes < 0 {
                self.minutes += 60;
                self.hours -= 1;
            }

            while self.hours < 0 {
                self.hours = self.hours + 24;
            }
        }
        Clock {
            hours: self.hours,
            minutes: self.minutes,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let minutes: i32 = self.minutes;
        let hour: i32 = self.hours;

        write!(f, "{:0>2}:{:0>2}", hour, minutes)
    }
}
