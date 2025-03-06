use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours , self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }


    pub fn div_euc(n: i32) ->i32 {
        n.div_euclid(60)
    }


    pub fn rem_euc(n: i32) ->i32 {
        n.rem_euclid(24)
    }

    // pub fn to_string(self) -> String {
    //     format!(
    //         "{hours:02}:{minutes:02}",
    //         hours = self.hours,
    //         minutes = self.minutes
    //     )
    // }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours==other.hours && self.minutes==other.minutes
    }
}

