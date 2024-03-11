use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut h = self.minutes / 60;
        let m = self.minutes - (h * 60);
        if h >= 24 {
            h = h % 24;
        }

        let mut time = "".to_string();

        if h < 10 {
            time += "0";
        }
        time += &format!("{}:", h).to_string();

        if m < 10 {
            time += "0";
        }
        time += &format!("{}", m).to_string();

        write!(f, "{}", time)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock { minutes: 0 };
        clock.add_time(hours, minutes)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        self.add_time(0, minutes)
    }

    pub fn add_time(mut self, hours: i32, minutes: i32) -> Self {
        let mut m = (hours * 60) + minutes + self.minutes;

        let full_day_minutes = 60 * 24;

        m = m % full_day_minutes;
        if m < 0 {
            m = full_day_minutes + m;
        }

        self.minutes = m;
        self
    }
}
