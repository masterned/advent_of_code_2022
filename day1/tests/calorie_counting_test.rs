use day1::calorie_counting::*;

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

    mod top_3 {
        use super::*;

        #[test]
        fn _troups_with_3_elves_should_return_all() {
            let mut troup = Troup::default();

            let elf1 = Elf::default();
            let elf2 = Elf::default();
            let elf3 = Elf::default();

            troup.add_elf(elf1);
            troup.add_elf(elf2);
            troup.add_elf(elf3);

            assert_eq!(
                get_top_3_heaviest_elves(&mut troup),
                [Elf::default(), Elf::default(), Elf::default()]
            );
        }

        #[test]
        fn _should_return_top_3_heaviest_elves() {
            let mut troup = Troup::default();

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

            troup.add_elf(elf1);
            troup.add_elf(elf2);
            troup.add_elf(elf3.clone());
            troup.add_elf(elf4.clone());
            troup.add_elf(elf5.clone());

            let heaviest_3 = vec![elf4, elf3, elf5];

            assert_eq!(get_top_3_heaviest_elves(&mut troup), heaviest_3);
        }
    }

    mod weight_totaling {
        use super::*;

        #[test]
        fn _should_return_total_weight_of_elves() {
            let mut elf3 = Elf::default();
            elf3.add_calories(5000);
            elf3.add_calories(6000);

            let mut elf4 = Elf::default();
            elf4.add_calories(7000);
            elf4.add_calories(8000);
            elf4.add_calories(9000);

            let mut elf5 = Elf::default();
            elf5.add_calories(10_000);

            let elves = vec![elf4, elf3, elf5];

            assert_eq!(get_total_elves_weight(&elves), 45_000);
        }
    }
}
