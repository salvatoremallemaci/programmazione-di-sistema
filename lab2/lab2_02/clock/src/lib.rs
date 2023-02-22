use std::fmt::{Debug, Display, Formatter};

pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_hours = hours;
        let mut new_minutes = minutes;

        while new_minutes < 0 {
            new_minutes = 60 + new_minutes;
            new_hours = new_hours - 1;
        }

        while new_hours < 0 {
            new_hours = 24 + new_hours;
        }

        while new_minutes > 60 {
            new_minutes = new_minutes - 60;
            new_hours = new_hours + 1;
        }

        while new_hours > 24 {
            new_hours = new_hours - 24;
        }

        if new_minutes == 60 {
            new_minutes = 0;
            new_hours = new_hours + 1;
        }

        if new_hours == 24 { new_hours = 0; }

        let c = Clock { hours: new_hours, minutes: new_minutes };
        println!("Clock created! {}", c);
        return c
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let current_hour = self.hours;
        let current_minute = self.minutes;
        let mut new_hour = current_hour;
        let mut new_minute = current_minute + minutes;
        if new_minute >= 60 {
            new_hour = new_hour + 1;
            if new_hour == 24 { new_hour = 0; }
            new_minute = new_minute - 60;
        }
        let new_clock = Clock::new(new_hour, new_minute);
        return new_clock;
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        if self.hours == other.hours && self.minutes == other.minutes {
            return true;
        } else { return false }
    }
}