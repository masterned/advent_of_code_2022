use day2::rock_paper_scissors::*;

#[test]
fn _rock_should_properly_compare_to_other_hands() {
    let rock1 = Hand::Rock;
    let rock2 = Hand::Rock;
    assert_eq!(rock1, rock2);

    let paper = Hand::Paper;
    assert!(rock1 < paper);

    let scissors = Hand::Scissors;
    assert!(rock1 > scissors);
}

#[test]
fn _paper_should_properly_compare_to_other_hands() {
    let paper1 = Hand::Paper;
    let paper2 = Hand::Paper;
    assert_eq!(paper1, paper2);

    let rock = Hand::Rock;
    assert!(paper1 > rock);

    let scissors = Hand::Scissors;
    assert!(paper1 < scissors);
}

#[test]
fn _scissors_should_properly_compare_to_other_hands() {
    let scissors1 = Hand::Scissors;
    let scissors2 = Hand::Scissors;
    assert_eq!(scissors1, scissors2);

    let rock = Hand::Rock;
    assert!(scissors1 < rock);

    let paper = Hand::Paper;
    assert!(scissors1 > paper);
}

#[test]
fn _hands_should_parse_from_strings() {
    assert_eq!(Ok(Hand::Rock), "A".parse::<Hand>());
    assert_eq!(Ok(Hand::Rock), "X".parse::<Hand>());

    assert_eq!(Ok(Hand::Paper), "B".parse::<Hand>());
    assert_eq!(Ok(Hand::Paper), "Y".parse::<Hand>());

    assert_eq!(Ok(Hand::Scissors), "C".parse::<Hand>());
    assert_eq!(Ok(Hand::Scissors), "Z".parse::<Hand>());
}

#[test]
fn _hands_should_convert_to_corresponding_scores() {
    assert_eq!(1, Hand::Rock as i32);
    assert_eq!(2, Hand::Paper as i32);
    assert_eq!(3, Hand::Scissors as i32);
}
