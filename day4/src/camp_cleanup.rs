use std::str::FromStr;

#[derive(Debug)]
pub struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    pub fn new(start: i32, end: i32) -> Self {
        Assignment { start, end }
    }

    pub fn contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

pub struct ParseAssignementErr;

impl FromStr for Assignment {
    type Err = ParseAssignementErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("-").collect();

        if parts.len() != 2 {
            return Err(Self::Err {});
        }

        if let (Some(start), Some(end)) = (parts.get(0), parts.get(1)) {
            let start: i32 = start.parse().map_err(|_| Self::Err {})?;
            let end: i32 = end.parse().map_err(|_| Self::Err {})?;
            Ok(Self { start, end })
        } else {
            Err(Self::Err {})
        }
    }
}

pub fn has_overlap(a1: &Assignment, a2: &Assignment) -> bool {
    a1.contains(a2) || a2.contains(a1)
}
