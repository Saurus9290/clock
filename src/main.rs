use std::fmt;

const DAY: i64 = 24 * 60; // Total minutes in a day
const HOUR: i64 = 60; // Total minutes in an hour

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    // Create a new clock with hours and minutes normalized to the range of a single day.
    pub fn new(hours: i64, minutes: i64) -> Clock {
        Clock {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY, // Normalize time to always be positive and within 24 hours
        }
    }

    // Add minutes to the current clock and return a new Clock instance.
    pub fn add_minutes(self, minutes: i64) -> Clock {
        Clock::new(0, self.minutes + minutes)
    }
}

// Implement Display trait to format the clock output as HH:MM
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}
fn main() {
    let clock = Clock::new(10, 45); // 10:45 AM
    println!("Current time: {}", clock); // Should print: 10:45

    let new_clock = clock.add_minutes(20); // Adds 20 minutes
    println!("New time: {}", new_clock); // Should print: 11:05
}
