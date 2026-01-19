#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct JavaVersion {
    pub major: u32,
}

impl JavaVersion {
    pub const JAVA_8: Self = Self { major: 8 };
    pub const JAVA_11: Self = Self { major: 11 };
    pub const JAVA_17: Self = Self { major: 17 };
    pub const JAVA_21: Self = Self { major: 21 };

    pub fn new(major: u32) -> Self {
        Self { major }
    }
}
