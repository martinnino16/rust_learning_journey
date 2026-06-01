use std::fmt;

const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i64,
}



impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        
        Self {
            minutes :(((hours * HOUR + minutes) % DAY) + DAY ) % DAY
        }
    }
  
    pub fn add_minutes(&self, minutes: i64) -> Self {
       Clock::new(0, self.minutes + minutes)
    }
}

