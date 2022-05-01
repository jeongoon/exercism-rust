use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;

pub fn unsafe_clock_div_mod(val: i32, divider_: i32) -> (i32, i32) {
    // which doesn't check divider could be zero
    let divider = divider_ as i32;

    let (mut quot, mut modu) = (val / divider, val % divider);

    if modu < 0 {
        modu += divider;
        quot -= 1;
    }
    (quot, modu)
}

// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{hours:0>2}:{minutes:0>2}",
            hours = &self.hours,
            minutes = &self.minutes
        )
    }
}

impl Clock {
    fn to_my_clock(hours: i32, minutes: i32) -> Self {
        let total_minutes =
            (hours * MINUTES_PER_HOUR + minutes).rem_euclid(HOURS_PER_DAY * MINUTES_PER_HOUR);

        let (new_hours, new_minutes) = unsafe_clock_div_mod(total_minutes, MINUTES_PER_HOUR);

        Clock {
            hours: new_hours as u8,
            minutes: new_minutes as u8,
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::to_my_clock(hours, minutes)
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let new_clock = Clock::to_my_clock(self.hours as i32, self.minutes as i32 + minutes);

        // update values
        *self = new_clock;
        *self
    }
}

fn main() {
    let mut clock = Clock::new(21, 37);
    clock.add_minutes(30);
    println!("{:?}", clock.to_string())
}
