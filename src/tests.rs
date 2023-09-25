#[cfg(test)]

use super::*;



#[allow(unused_macros)]
macro_rules! assert_calculate_heuristic {
    ($s:expr, $v:expr) => {
        {
            let field = field::Field::from($s).ok().unwrap();
            let result: f32 = astar::calculate_heuristic(&field);
            assert_eq!(result, $v as f32, "{} is {}, should be {}", $s, result, $v as f32);
        }
    };
}

#[test]
fn test_calculate_heuristic() {
    assert_calculate_heuristic!(0x5134207896ACDEBF, 12);
}



#[allow(unused_macros)]
macro_rules! assert_calculate_heuristic_change {
    ($s:expr, $m:expr, $v:expr) => {
        {
            let field = field::Field::from($s).ok().unwrap();
            let result: f32 = astar::calculate_heuristic_change(&field, $m);
            assert_eq!(result, $v as f32, "{} is {}, should be {}", $s, result, $v as f32);
        }
    };
}

#[test]
fn test_calculate_heuristic_change() {
    assert_calculate_heuristic_change!(0x5134207896ACDEBF, field::Move::Up, 2);
    assert_calculate_heuristic_change!(0x5134207896ACDEBF, field::Move::Down, -2);
    assert_calculate_heuristic_change!(0x5134207896ACDEBF, field::Move::Left, 0);
    assert_calculate_heuristic_change!(0x5134207896ACDEBF, field::Move::Right, 0);

    assert_calculate_heuristic_change!(0x1723068459ACDEBF, field::Move::Up, 2);
    assert_calculate_heuristic_change!(0x1723068459ACDEBF, field::Move::Down, -2);
    assert_calculate_heuristic_change!(0x1723068459ACDEBF, field::Move::Right, 0);

    assert_calculate_heuristic_change!(0x123456789ABCDEF0, field::Move::Up, 2);
    assert_calculate_heuristic_change!(0x123456789ABCDEF0, field::Move::Left, 2);

    assert_calculate_heuristic_change!(0x12345678EABC0D9F, field::Move::Up, 0);
    assert_calculate_heuristic_change!(0x12345678EABC0D9F, field::Move::Right, -2);
}



#[allow(unused_macros)]
macro_rules! is_solvable {
    ($s:expr, $b:expr) => {
        {
            let field = field::Field::from($s).ok().unwrap();
            let result: bool = field.is_solvable();
            assert_eq!(result, $b, "{} is {}, should be {}", $s, result, $b);
        }
    };
}


#[test]
fn test_is_solvable() {
    is_solvable!(0x123456789AFB0EDC, false);
    is_solvable!(0x123456789AFB0EDC, false);
    is_solvable!(0xF2345678A0BE91CD, false);
    is_solvable!(0x123456789ABCDEF0,  true);
    is_solvable!(0x1234067859ACDEBF,  true);
    is_solvable!(0x5134207896ACDEBF,  true);
    is_solvable!(0x16245A3709C8DEBF,  true);
    is_solvable!(0x1723068459ACDEBF,  true);
    is_solvable!(0x12345678A0BE9FCD,  true);
    is_solvable!(0x51247308A6BE9FCD,  true);
    is_solvable!(0xF2345678A0BE91DC,  true);
    is_solvable!(0x75123804A6BE9FCD,  true);
    is_solvable!(0x75AB2C416D389F0E,  true);
    is_solvable!(0x04582E1DF79BCA36,  true);
    is_solvable!(0xFE169B4C0A73D852,  true);
    is_solvable!(0xD79F2E8A45106C3B,  true);
    is_solvable!(0xDBE87A2C91F65034,  true);
    is_solvable!(0xBAC0F478E19623D5,  true);
}


#[allow(unused_macros)]
macro_rules! solve {
    ($s:expr, $b:expr) => {
        {
            let field = field::Field::from($s).ok().unwrap();
            let mut star = astar::AStar::new(&field);
            let mut result_option: Option<Result<AStarSolution, ()>> = Default::default();
            while result_option.is_none() {
                result_option = star.step();
            }

            let solution = result_option.unwrap().ok().unwrap();
            let length = solution.moves.len();
            assert_eq!(length, $b, "{} is {}, should be {}", $s, length, $b);
        }
    };
}

#[test]
fn test_solver() {
    solve!(0x123456789ABCDEF0,  0);
    solve!(0x1234067859ACDEBF,  5);
    solve!(0x5134207896ACDEBF,  8);
    solve!(0x16245A3709C8DEBF, 10);
    solve!(0x1723068459ACDEBF, 13);
    solve!(0x12345678A0BE9FCD, 19);
    solve!(0x51247308A6BE9FCD, 27);
    solve!(0xF2345678A0BE91DC, 33);
    solve!(0x75123804A6BE9FCD, 35);
    solve!(0x75AB2C416D389F0E, 45);
    // solve!(0x04582E1DF79BCA36, 48);
    // solve!(0xFE169B4C0A73D852, 52);
    // solve!(0xD79F2E8A45106C3B, 55);
    // solve!(0xDBE87A2C91F65034, 58);
    // solve!(0xBAC0F478E19623D5, 61);
}