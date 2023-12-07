use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Default)]
pub struct Tuple(pub usize, pub usize, pub usize);

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Tuple {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Tuple {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}
