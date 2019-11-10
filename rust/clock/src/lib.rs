trait Modular {
    fn modulus(&self, other: Self) -> Self;
}

impl Modular for i32 {
    fn modulus(&self, other: Self) -> Self {
        (self % other + other) % other
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{minutes: (hours * 60 + minutes).modulus(24 * 60)}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

