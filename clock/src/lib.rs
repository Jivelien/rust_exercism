#[derive(Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{hours, minutes}
    }

    pub fn to_string(self) -> String {
        format!("{hours:02}:{minutes:02}", hours = self.hours, minutes = self.minutes)
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
