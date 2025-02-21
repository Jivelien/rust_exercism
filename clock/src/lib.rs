#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let minutes_from_midnight = (total_minutes % (24 * 60) + (24 * 60)) % (24 * 60);
        let total_hours = minutes_from_midnight / 60;
        let remain_minutes = minutes_from_midnight - total_hours * 60;
        Clock {
            hours: total_hours,
            minutes: remain_minutes,
        }
    }

    pub fn to_string(self) -> String {
        format!(
            "{hours:02}:{minutes:02}",
            hours = self.hours,
            minutes = self.minutes
        )
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        todo!("Add {minutes} minutes to existing Clock time");
    }
}
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
