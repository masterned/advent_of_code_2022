use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub enum Altitude {
    Start,
    Height(usize),
    End,
    None,
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
                    Altitude::None => false,
                },
                Altitude::Start => other == &Altitude::Height(1),
                _ => false,
            })
    }
}

impl PartialEq for Altitude {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Altitude::None => false,
            Altitude::Height(self_height) => {
                if let Altitude::Height(other_height) = other {
                    self_height == other_height
                } else {
                    false
                }
            }
            Altitude::Start => matches!(other, Altitude::Start),
            Altitude::End => matches!(other, Altitude::End),
        }
    }
}

impl PartialOrd for Altitude {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Altitude::Start => match other {
                Altitude::Start => Some(Ordering::Equal),
                Altitude::None => None,
                _ => Some(Ordering::Less),
            },
            Altitude::Height(self_height) => match other {
                Altitude::Start => Some(Ordering::Greater),
                Altitude::Height(other_height) => self_height.partial_cmp(other_height),
                Altitude::End => Some(Ordering::Less),
                Altitude::None => None,
            },
            Altitude::End => match other {
                Altitude::End => Some(Ordering::Equal),
                Altitude::None => None,
                _ => Some(Ordering::Greater),
            },
            Altitude::None => None,
        }
    }
}

impl From<char> for Altitude {
    fn from(value: char) -> Self {
        match value {
            'S' => Altitude::Start,
            'E' => Altitude::End,
            c => {
                if let Some(height) = (c as usize).checked_sub(96) {
                    Altitude::Height(height)
                } else {
                    Altitude::None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _should_create_altitude_from_char() {
        let start = Altitude::from('S');
        assert_eq!(start, Altitude::Start);

        let end = Altitude::from('E');
        assert_eq!(end, Altitude::End);

        let none = Altitude::from('?');
        assert!(matches!(none, Altitude::None));

        let h0 = Altitude::from('s');
        assert_eq!(h0, Altitude::Height(19));

        let h1 = Altitude::from('z');
        assert_eq!(h1, Altitude::Height(26));
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

    #[test]
    fn _should_not_be_able_to_compare_altitude_of_none() {
        let none0 = Altitude::None;
        let none1 = Altitude::None;
        let start = Altitude::Start;

        assert_eq!(none0.partial_cmp(&none1), None);
        assert_eq!(none0.partial_cmp(&start), None);
    }

    mod can_reach {
        use super::*;

        #[test]
        fn _none_should_never_reach() {
            let none = Altitude::None;

            let start = Altitude::Start;
            let end = Altitude::End;
            let height = Altitude::Height(1);

            assert!(!none.can_reach(&start));
            assert!(!none.can_reach(&end));
            assert!(!none.can_reach(&height));
        }

        #[test]
        fn _none_should_never_be_reached() {
            let none = Altitude::None;

            let start = Altitude::Start;
            let end = Altitude::End;
            let height = Altitude::Height(1);

            assert!(!start.can_reach(&none));
            assert!(!end.can_reach(&none));
            assert!(!height.can_reach(&none));
        }

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
        fn _should_return_true_if_destination_is_one_higher() {
            let start = Altitude::Start;
            let h0 = Altitude::Height(1);
            let h1 = Altitude::Height(2);
            let h2 = Altitude::from('z');
            let end = Altitude::End;

            assert!(start.can_reach(&h0));
            assert!(h0.can_reach(&h1));
            assert!(h2.can_reach(&end));
        }

        #[test]
        fn _end_should_always_be_able_to_reach_other_altitudes() {
            let start = Altitude::Start;
            let h0 = Altitude::Height(1);
            let h1 = Altitude::Height(2);
            let h2 = Altitude::from('z');
            let h3 = Altitude::Height(9001);
            let end = Altitude::End;

            assert!(end.can_reach(&start));
            assert!(end.can_reach(&h0));
            assert!(end.can_reach(&h1));
            assert!(end.can_reach(&h2));
            assert!(end.can_reach(&h3));
        }

        #[test]
        fn _other_altitudes_should_always_be_able_to_reach_start() {
            let start = Altitude::Start;
            let h0 = Altitude::Height(1);
            let h1 = Altitude::Height(2);
            let h2 = Altitude::from('z');
            let h3 = Altitude::Height(9001);
            let end = Altitude::End;

            assert!(h0.can_reach(&start));
            assert!(h1.can_reach(&start));
            assert!(h2.can_reach(&start));
            assert!(h3.can_reach(&start));
            assert!(end.can_reach(&start));
        }

        #[test]
        fn _should_return_false_if_destination_more_than_one_higher() {
            let start = Altitude::Start;
            let h0 = Altitude::Height(1);
            let h1 = Altitude::Height(2);
            let h2 = Altitude::from('z');
            let end = Altitude::End;

            assert!(!start.can_reach(&h1));
            assert!(!start.can_reach(&h2));
            assert!(!start.can_reach(&end));

            assert!(!h0.can_reach(&h2));
            assert!(!h0.can_reach(&end));

            assert!(!h1.can_reach(&end));
        }
    }
}
