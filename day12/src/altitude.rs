#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Altitude {
    Start,
    Height(usize),
    End,
}

impl Altitude {
    #[must_use]
    pub fn can_reach(&self, other: &Altitude) -> bool {
        self >= other
            || (match self {
                Altitude::Height(self_height) => match other {
                    Altitude::Start => true,
                    Altitude::End => *self_height >= 26,
                    Altitude::Height(other_height) => *other_height == self_height + 1,
                },
                Altitude::Start => other == &Altitude::Height(1),
                _ => false,
            })
    }
}

impl TryFrom<char> for Altitude {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Altitude::Start),
            'E' => Ok(Altitude::End),
            c => {
                if let Some(height) = (c as usize).checked_sub(96) {
                    Ok(Altitude::Height(height))
                } else {
                    Err("Cannot parse Altitude")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _should_create_altitude_from_char() -> Result<(), &'static str> {
        let start = Altitude::try_from('S')?;
        assert_eq!(start, Altitude::Start);

        let end = Altitude::try_from('E')?;
        assert_eq!(end, Altitude::End);

        let none = Altitude::try_from('?').ok();
        assert_eq!(none, None);

        let h0 = Altitude::try_from('s')?;
        assert_eq!(h0, Altitude::Height(19));

        let h1 = Altitude::try_from('z')?;
        assert_eq!(h1, Altitude::Height(26));

        Ok(())
    }

    #[test]
    fn _should_be_able_to_compare_altitudes() {
        let start = Altitude::Start;
        let end = Altitude::End;
        let middle0 = Altitude::Height(42);
        let middle1 = Altitude::Height(69);

        assert!(start < end);
        assert!(start < middle0);
        assert!(middle0 < middle1);
        assert!(middle1 < end);
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
        fn _should_return_true_if_destination_is_one_higher() -> Result<(), &'static str> {
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
        fn _end_should_always_be_able_to_reach_other_altitudes() -> Result<(), &'static str> {
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
        fn _other_altitudes_should_always_be_able_to_reach_start() -> Result<(), &'static str> {
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
        fn _should_return_false_if_destination_more_than_one_higher() -> Result<(), &'static str> {
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
