use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minute: i16
}

impl Clock {
    pub fn new(hour: i16, minute: i16) -> Clock {
        Clock {
            minute: Self::clock_minutes((hour * 60) + minute)
        }
    }

    pub fn add_minutes(&mut self, min: i16) -> Clock {
        Clock {
            minute: Self::clock_minutes(self.minute + min)
        }
    }

    pub fn hours(&self) -> i16 {
        self.minute / 60
    }

    pub fn minutes(&self) -> i16 {
        self.minute % 60
    }

    fn clock_minutes(min: i16) -> i16 {
        (1440 + (min % 1440)) % 1440
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}