use std::{error::Error, fmt, ops};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Altitude {
    Start,
    Height(usize),
    End,
}

impl Altitude {
    #[must_use]
    pub fn can_reach(&self, other: &Altitude) -> bool {
        self >= &(*other - 1)
    }
}

#[derive(Debug)]
pub enum ParseError {
    CharacterOutOfRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Character outside of Altitude range.")
    }
}

impl Error for ParseError {}

impl TryFrom<char> for Altitude {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Altitude::Start),
            'E' => Ok(Altitude::End),
            c if c.is_alphabetic() && c.is_lowercase() => Ok(Self::Height(c as usize - 96)),
            _ => Err(ParseError::CharacterOutOfRange),
        }
    }
}

impl ops::Sub<usize> for Altitude {
    type Output = Altitude;

    fn sub(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return self;
        }

        match self {
            Altitude::Height(lhs) if lhs > rhs + 1 => Altitude::Height(lhs - rhs),
            Altitude::End if rhs <= 25 => Altitude::Height(26 - rhs),
            _ => Altitude::Start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod try_from {
        use super::*;

        #[test]
        fn _should_create_ok_altitude_from_lowercase_letter_uppercase_s_or_e(
        ) -> Result<(), Box<dyn Error>> {
            let start = Altitude::try_from('S')?;
            assert_eq!(start, Altitude::Start);

            let end = Altitude::try_from('E')?;
            assert_eq!(end, Altitude::End);

            let h0 = Altitude::try_from('s')?;
            assert_eq!(h0, Altitude::Height(19));

            let h1 = Altitude::try_from('z')?;
            assert_eq!(h1, Altitude::Height(26));

            Ok(())
        }

        #[test]
        fn _should_return_error_if_char_not_lowercase_letter_uppercase_s_or_e() {
            let other_capital_altitude = Altitude::try_from('A');
            assert!(matches!(
                other_capital_altitude,
                Err(ParseError::CharacterOutOfRange)
            ));

            let punctuation = Altitude::try_from('?');
            assert!(matches!(punctuation, Err(ParseError::CharacterOutOfRange)));

            let number = Altitude::try_from('4');
            assert!(matches!(number, Err(ParseError::CharacterOutOfRange)));
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn _anything_sub_0_should_be_itself() {
            let start = Altitude::Start;
            let result = start - 0;
            assert_eq!(result, Altitude::Start);

            let height = Altitude::Height(5);
            let result = height - 0;
            assert_eq!(result, Altitude::Height(5));

            let end = Altitude::End;
            let result = end - 0;
            assert_eq!(result, Altitude::End);
        }

        #[test]
        fn _subtracting_anything_from_start_should_return_start() {
            let start = Altitude::Start;

            let result = start - 1;
            assert_eq!(result, Altitude::Start);

            let result = start - 18;
            assert_eq!(result, Altitude::Start);

            let result = start - 200;
            assert_eq!(result, Altitude::Start);

            let result = start - 520;
            assert_eq!(result, Altitude::Start);

            let result = start - 9001;
            assert_eq!(result, Altitude::Start);
        }

        #[test]
        fn _subtracting_value_greater_than_or_equal_to_height_should_return_start() {
            let height = Altitude::Height(1);
            let result = height - 1;
            assert_eq!(result, Altitude::Start);

            let height = Altitude::Height(1);
            let result = height - 100;
            assert_eq!(result, Altitude::Start);

            let height = Altitude::Height(10);
            let result = height - 11;
            assert_eq!(result, Altitude::Start);
        }

        #[test]
        fn _subtracting_27_or_greater_from_end_should_return_start() {
            let end = Altitude::End;

            let result = end - 27;
            assert_eq!(result, Altitude::Start);

            let result = end - 30;
            assert_eq!(result, Altitude::Start);

            let result = end - 9001;
            assert_eq!(result, Altitude::Start);
        }

        #[test]
        fn _subtracting_value_less_than_height_should_return_new_height() {
            let height = Altitude::Height(26);

            let result = height - 1;
            assert_eq!(result, Altitude::Height(25));

            let result = height - 13;
            assert_eq!(result, Altitude::Height(13));

            let result = height - 25;
            assert_eq!(result, Altitude::Height(1));
        }

        #[test]
        fn _subtracting_less_than_27_from_end_should_return_new_height() {
            let end = Altitude::End;

            let result = end - 1;
            assert_eq!(result, Altitude::Height(26));

            let result = end - 14;
            assert_eq!(result, Altitude::Height(13));

            let result = end - 26;
            assert_eq!(result, Altitude::Height(1));
        }
    }

    mod can_reach {
        use super::*;

        #[test]
        fn _should_return_true_if_equal() {
            let s0 = Altitude::Start;
            let s1 = Altitude::Start;

            assert!(s0.can_reach(&s1));
            assert!(s1.can_reach(&s0));

            let a0 = Altitude::Height(12);
            let a1 = Altitude::Height(12);

            assert!(a0.can_reach(&a1));
            assert!(a1.can_reach(&a0));

            let e0 = Altitude::End;
            let e1 = Altitude::End;

            assert!(e0.can_reach(&e1));
            assert!(e1.can_reach(&e0));
        }

        #[test]
        fn _should_return_true_if_destination_is_lower() {
            let start = Altitude::Start;
            let a0 = Altitude::Height(1);
            let a1 = Altitude::Height(9001);
            let end = Altitude::End;

            assert!(end.can_reach(&a1));
            assert!(end.can_reach(&a0));
            assert!(end.can_reach(&start));

            assert!(a1.can_reach(&a0));
            assert!(a1.can_reach(&start));

            assert!(a0.can_reach(&start));
        }

        #[test]
        fn _should_return_true_if_destination_is_one_higher() -> Result<(), Box<dyn Error>> {
            let start = Altitude::Start;
            let h0 = Altitude::Height(1);
            let h1 = Altitude::Height(2);
            let h2 = Altitude::try_from('z')?;
            let end = Altitude::End;

            assert!(start.can_reach(&h0));
            assert!(h0.can_reach(&h1));
            assert!(h2.can_reach(&end));

            Ok(())
        }

        #[test]
        fn _end_should_always_be_able_to_reach_other_altitudes() -> Result<(), Box<dyn Error>> {
            let start = Altitude::Start;
            let h0 = Altitude::Height(1);
            let h1 = Altitude::Height(2);
            let h2 = Altitude::try_from('z')?;
            let h3 = Altitude::Height(9001);
            let end = Altitude::End;

            assert!(end.can_reach(&start));
            assert!(end.can_reach(&h0));
            assert!(end.can_reach(&h1));
            assert!(end.can_reach(&h2));
            assert!(end.can_reach(&h3));

            Ok(())
        }

        #[test]
        fn _other_altitudes_should_always_be_able_to_reach_start() -> Result<(), Box<dyn Error>> {
            let start = Altitude::Start;
            let h0 = Altitude::Height(1);
            let h1 = Altitude::Height(2);
            let h2 = Altitude::try_from('z')?;
            let h3 = Altitude::Height(9001);
            let end = Altitude::End;

            assert!(h0.can_reach(&start));
            assert!(h1.can_reach(&start));
            assert!(h2.can_reach(&start));
            assert!(h3.can_reach(&start));
            assert!(end.can_reach(&start));

            Ok(())
        }

        #[test]
        fn _should_return_false_if_destination_more_than_one_higher() -> Result<(), Box<dyn Error>>
        {
            let start = Altitude::Start;
            let h0 = Altitude::Height(1);
            let h1 = Altitude::Height(2);
            let h2 = Altitude::try_from('z')?;
            let end = Altitude::End;

            assert!(!start.can_reach(&h1));
            assert!(!start.can_reach(&h2));
            assert!(!start.can_reach(&end));

            assert!(!h0.can_reach(&h2));
            assert!(!h0.can_reach(&end));

            assert!(!h1.can_reach(&end));

            Ok(())
        }
    }
}
