use core::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hrs: i32,
    min: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let minutes_left_after_24hrs_ie_1440min = total_minutes.rem_euclid(1440);
        let hours_under_24 = minutes_left_after_24hrs_ie_1440min / 60;
        let minutes_under_60min = minutes_left_after_24hrs_ie_1440min % 60;

        Self {
            hrs: hours_under_24,
            min: minutes_under_60min,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hrs, self.min + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hrs, self.min)
    }
}
