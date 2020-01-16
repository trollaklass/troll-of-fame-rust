pub mod elf {

    #[cfg(test)]
    use proptest_derive::Arbitrary;
    #[repr(u32)]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Eq)]
    #[cfg_attr(test, derive(Arbitrary))]
    pub enum Race {
        Dark = 1,
        High = 2,
    }

    #[repr(u32)]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Eq)]
    #[cfg_attr(test, derive(Arbitrary))]
    pub enum Role {
        Swordsman = 1,
        Archer = 2,
        Priest = 5,
        Warlock = 4,
    }

    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash, Eq)]
    #[cfg_attr(test, derive(Arbitrary))]
    pub struct Elf {
        race: Race,
        role: Role,
    }

    impl Elf {
        pub fn new(role: Role, race: Race) -> Self {
            Elf { race, role }
        }

        pub fn value(&self) -> u32 {
            *&self.race as u32 * *&self.role as u32
        }
    }

    #[cfg(test)]
    mod unit_tests {
        use super::*;
        #[test]
        fn two_warlock_dark_elves_should_be_equal() {
            let doomshadow = Elf::new(Role::Warlock, Race::Dark);
            let thundershade = Elf::new(Role::Warlock, Race::Dark);
            assert_eq!(doomshadow, thundershade);
        }
        #[test]
        fn archer_dark_and_swordsman_high_values_should_be_equal() {
            let faeor = Elf::new(Role::Archer, Race::Dark).value();
            let shadowblight = Elf::new(Role::Swordsman, Race::High).value();
            assert_eq!(faeor, shadowblight);
        }

    }

    #[cfg(test)]
    mod property_tests {
        use super::*;
        use proptest::prelude::*;
        proptest! {
            #[test]
            fn elf_value_should_be_positive(elf : Elf) {
                assert!(elf.value() > 0);
            }
            // #[test]
            // fn high_elf_value_should_be_even(role : Role) {
            //     let elf = Elf::new(role, Race::High);
            //     assert_eq!(elf.value() % 2, 0);
            // }
        }
    }

}
