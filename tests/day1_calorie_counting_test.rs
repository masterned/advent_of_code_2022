use advent_of_code_2022::day1_calorie_counting::*;

mod elf_struct {
    use super::*;

    #[test]
    fn _elf_with_no_calories_should_weigh_0() {
        let weightless_elf = Elf::default();
        assert_eq!(weightless_elf.total_calories(), 0);
    }

    #[test]
    fn _elf_with_one_calorie_value_should_weigh_the_value() {
        let mut single_value_elf = Elf::default();
        single_value_elf.add_calories(42);
        assert_eq!(single_value_elf.total_calories(), 42);
    }
}

mod elf_functions {
    use super::*;

    #[test]
    fn _vec_with_no_elves_should_return_none() {
        let nega_elves: Vec<Elf> = Vec::new();
        assert_eq!(get_heaviest_elf(&nega_elves), None);
    }

    #[test]
    fn _vec_with_one_elf_should_return_the_elf() {
        let elf = Elf::default();
        let single_elf = vec![Elf::default()];

        assert_eq!(get_heaviest_elf(&single_elf), Some(&elf));
    }

    #[test]
    fn _vec_with_multiple_elves_should_return_elf_with_most_calories() {
        let mut elf1 = Elf::default();
        elf1.add_calories(1000);
        elf1.add_calories(2000);
        elf1.add_calories(3000);

        let mut elf2 = Elf::default();
        elf2.add_calories(4000);

        let mut elf3 = Elf::default();
        elf3.add_calories(5000);
        elf3.add_calories(6000);

        let mut elf4 = Elf::default();
        elf4.add_calories(7000);
        elf4.add_calories(8000);
        elf4.add_calories(9000);

        let mut elf5 = Elf::default();
        elf5.add_calories(10_000);

        let elves = vec![elf1, elf2, elf3, elf4, elf5];

        let mut heaviest_elf = Elf::default();
        heaviest_elf.add_calories(7000);
        heaviest_elf.add_calories(8000);
        heaviest_elf.add_calories(9000);

        assert_eq!(get_heaviest_elf(&elves), Some(&heaviest_elf));
    }
}
