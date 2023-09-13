#[cfg(test)]

use super::*;

#[allow(unused_macros)]
macro_rules! is_solvable {
    ($s:expr, $b:expr) => {
        {
            let result: bool = field::Field::from($s).is_solvable();
            assert_eq!(result, $b, "{} is {}, should be {}", $s, result, $b);
        }
    };
}

#[test]
fn test_is_solvable() {
    is_solvable!("123456789AFB0EDC", false);
    is_solvable!("123456789AFB0EDC", false);
    is_solvable!("F2345678A0BE91CD", false);
    is_solvable!("123456789ABCDEF0",  true);
    is_solvable!("1234067859ACDEBF",  true);
    is_solvable!("5134207896ACDEBF",  true);
    is_solvable!("16245A3709C8DEBF",  true);
    is_solvable!("1723068459ACDEBF",  true);
    is_solvable!("12345678A0BE9FCD",  true);
    is_solvable!("51247308A6BE9FCD",  true);
    is_solvable!("F2345678A0BE91DC",  true);
    is_solvable!("75123804A6BE9FCD",  true);
    is_solvable!("75AB2C416D389F0E",  true);
    is_solvable!("04582E1DF79BCA36",  true);
    is_solvable!("FE169B4C0A73D852",  true);
    is_solvable!("D79F2E8A45106C3B",  true);
    is_solvable!("DBE87A2C91F65034",  true);
    is_solvable!("BAC0F478E19623D5",  true);
}