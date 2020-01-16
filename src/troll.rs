pub mod troll {
    use crate::elf::*;
    use std::collections::HashMap;

    #[cfg(test)]
    use proptest_derive::Arbitrary;

    type Score = u32;
    type Counter = u32;
    type Kill = i32;

    #[derive(Debug, PartialEq, Clone, Eq)]
    #[cfg_attr(test, derive(Arbitrary))]
    pub struct Troll {
        name: String,
        kills: HashMap<elf::Elf, Counter>,
    }

    impl Troll {
        pub fn new(name: String) -> Self {
            let kills = HashMap::new();
            Troll { name, kills }
        }

        pub fn i_got(self, k: Kill, e: elf::Elf) -> Self {
            let mut new_kills = self.kills.clone();
            let new_score = *self.kills.get(&e).unwrap_or(&0) as Kill + k;
            if new_score <= 0 {
                self.all_elves_of_a_kind_resurrected(e)
            } else {
                new_kills.insert(e, new_score as Counter);
                Troll {
                    name: self.name.clone(),
                    kills: new_kills,
                }
            }
        }

        pub fn i_got_one(self, e: elf::Elf) -> Self {
            self.i_got(1, e)
        }

        pub fn oops_he_survived(self, e: elf::Elf) -> Self {
            self.i_got(-1, e)
        }

        pub fn all_elves_of_a_kind_resurrected(self, e: elf::Elf) -> Self {
            let mut new_kills = self.kills.clone();
            new_kills.remove(&e);
            Troll {
                name: self.name.clone(),
                kills: new_kills,
            }
        }

        pub fn all_elves_resurrected(self) -> Self {
            Troll::new(self.name.clone())
        }

        pub fn scoring(self) -> Score {
            self.kills.iter().fold(0, |_acc, (k, v)| k.value() * v)
        }
    }

    #[cfg(test)]
    mod unit_tests {
        use super::*;
        use crate::elf::elf::*;
        #[test]
        fn i_got_5_faeor_should_add_key_faeor_with_value_5() {
            let trollay_before = Troll::new(String::from("Trollay"));
            let faeor = Elf::new(Role::Archer, Race::Dark);
            let trollay_after = trollay_before.i_got(5, faeor);
            assert_eq!(trollay_after.kills.get(&faeor), Some(&5));
        }
        #[test]
        fn i_got_one_faeor_should_add_key_faeor_with_value_1() {
            let trollay_before = Troll::new(String::from("Trollay"));
            let faeor = Elf::new(Role::Archer, Race::Dark);
            let trollay_after = trollay_before.i_got_one(faeor);
            assert_eq!(trollay_after.kills.get(&faeor), Some(&1));
        }
        #[test]
        fn troll_after_oops_he_survived_should_be_the_same_as_before() {
            let trollay = Troll::new(String::from("Trollay"));
            let trollay_before = trollay.clone();
            let faeor = Elf::new(Role::Archer, Race::Dark);
            let trollay_after = trollay.i_got_one(faeor).oops_he_survived(faeor);
            assert_eq!(trollay_before, trollay_after);
        }
        #[test]
        fn got_one_when_no_kill_should_have_one_kill() {
            let trollay_before = Troll::new(String::from("Trollay"));
            let faeor = Elf::new(Role::Archer, Race::Dark);
            let trollay_after = trollay_before.i_got_one(faeor);
            assert_eq!(trollay_after.kills.len(), 1);
        }
        #[test]
        fn scoring_should_be_0_when_create_a_new_troll() {
            let trollay = Troll::new(String::from("Trollay"));
            assert_eq!(trollay.scoring(), 0);
        }
        #[test]
        fn scoring_should_be_0_when_a_warlock_ressurect_every_elves() {
            let trollay_before = Troll::new(String::from("Trollay"));
            let faeor = Elf::new(Role::Archer, Race::Dark);
            let trollay_after = trollay_before
                .i_got_one(faeor)
                .i_got(5, faeor)
                .all_elves_resurrected();
            assert_eq!(trollay_after.scoring(), 0);
        }

    }

    #[cfg(test)]
    mod property_tests {
        use super::*;
        use crate::elf::elf::*;
        use proptest::prelude::*;
        proptest! {
            // INVARIANCE
            #[test]
            fn troll_scoring_should_be_0_when_all_elves_resurrected(troll : Troll) {
                assert_eq!(troll.all_elves_resurrected().scoring(), 0);
            }

            #[test]
            fn troll_scoring_should_always_be_greater_greater_or_equal_0(_troll : Troll) {
                // Test go there
            }

            // INVERSE
            #[test]
            fn oops_he_survived_should_always_be_inverse_of_i_got_one(_troll : Troll, _elf : Elf) {
                // Test go there
            }

            // ANALOGY
            #[test]
            fn i_got_one_and_i_got_should_be_consistent(_troll : Troll, _elf : Elf) {
                // Test go there
            }

            // IDEMPOTENCE
            #[test]
            fn all_elves_of_a_kind_resurrected_brings_the_troll_kills_to_a_stable_state(_troll : Troll, _elf : Elf) {
                // Test go there
            }


            // METAMORPHISM


            // INJECTION
        }
    }

}
