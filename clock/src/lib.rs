use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

const HOUR_MINUTES: i32 = 60;
const DAY_MINUTES: i32 = 60 * 24;

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = self.minutes / 60;
        let m = self.minutes - (h * 60);

        write!(f, "{:02}:{:02}", h, m)
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
        let mut m = (hours * HOUR_MINUTES) + minutes + self.minutes;

        m = m % DAY_MINUTES;
        if m < 0 {
            m = DAY_MINUTES + m;
        }

        self.minutes = m;
        self
    }
}
