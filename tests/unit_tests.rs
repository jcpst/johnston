extern crate johnston;
extern crate rug;
mod intext;
use johnston::*;
use rug::Rational;
use std::f32;

#[test]
fn it_returns_factors() {
    // arrange
    let six: IntExt = 6;
    let seven = 7;
    let thirty_four = 34;

    // assert
    assert_eq!(six.factors(), vec![1, 2, 3, 6]);
    assert_eq!(seven.factors(), vec![1, 7]);
    assert_eq!(thirty_four.factors(), vec![1, 2, 17, 34]);
}

#[test]
fn it_returns_the_greatest_prime_factor() {
    // arrange
    let fifteen = 15;
    let thirty_four = 34;
    let fourty_nine = 49;

    // assert
    assert_eq!(fifteen.gpf(), 5);
    assert_eq!(thirty_four.gpf(), 17);
    assert_eq!(fourty_nine.gpf(), 7);
}

#[test]
fn it_returns_a_list_from_a_ratio() {
    // arrange
    let a = Rational::from((3, 2));

    // assert
    assert_eq!(a.to_list(), vec![3, 2]);
}

#[test]
fn it_is_a_power_of_two() {
    // arrange
    let a = 3;
    let b = 8;
    let c = 17;
    let d = 1024;
    let e = 1111;

    // assert
    assert_eq!(a.is_power_of_two(), false);
    assert_eq!(b.is_power_of_two(), true);
    assert_eq!(c.is_power_of_two(), false);
    assert_eq!(d.is_power_of_two(), true);
    assert_eq!(e.is_power_of_two(), false);
}

#[test]
fn it_calculates_cents() {
    // arrange
    let fifth = Rational::from((3, 2));

    // act
    let a = fifth.cents();

    // assert
    let abs_difference = (a.abs() - a).abs();
    assert!(abs_difference <= f32::EPSILON);
}

#[test]
fn it_returns_the_correct_ordinal() {
    // arrange
    let a = Rational::from((3, 2));
    let b = Rational::from((7, 5));
    let c = Rational::from((4, 3));

    // assert
    assert_eq!(a.ordinal(), Ordinal::Otonal);
    assert_eq!(b.ordinal(), Ordinal::Otonal);
    assert_eq!(c.ordinal(), Ordinal::Utonal);
}

#[test]
fn it_gets_the_correct_limit() {
    // arrange
    let a = Rational::from((3, 2));
    let b = Rational::from((5, 4));
    let c = Rational::from((7, 5));
    let d = Rational::from((13, 8));

    // assert
    assert_eq!(a.limit(), 3);
    assert_eq!(b.limit(), 5);
    assert_eq!(c.limit(), 7);
    assert_eq!(d.limit(), 13);
}

#[test]
fn it_returns_the_interval_to_traverse_the_lattice() {
    assert_eq!(lattice_relation(3, Ordinal::Otonal), Rational::from((3, 2)));
    assert_eq!(lattice_relation(5, Ordinal::Otonal), Rational::from((5, 4)));
    assert_eq!(lattice_relation(3, Ordinal::Utonal), Rational::from((4, 3)));
    assert_eq!(lattice_relation(5, Ordinal::Utonal), Rational::from((8, 5)));
}

#[test]
fn it_flattens() {
    // arrange
    let a = Rational::from((18, 4));
    let b = Rational::from((9, 4));
    let c = Rational::from((6, 2));
    let d = Rational::from((3, 4));

    // assert
    assert_eq!(a.flatten(), (9, 8));
    assert_eq!(b.flatten(), (9, 8));
    assert_eq!(c.flatten(), (3, 2));
    assert_eq!(d.flatten(), (3, 2));
}

#[test]
fn it_steps_otonally() {
    // arrange
    let fifth = Rational::from((3, 2));
    let third = Rational::from((5, 4));

    // act
    let nine_eighths = otonal_step(&fifth, &fifth);
    let fifteen_eighths = otonal_step(&third, &fifth);

    // assert
    assert_eq!(nine_eighths, (9, 8));
    assert_eq!(fifteen_eighths, (15, 8));
}

#[test]
fn it_steps_utonally() {
    // arrange
    let fifth = Rational::from((3, 2));
    let third = Rational::from((5, 4));

    // act
    let one = utonal_step(&fifth, &fifth);
    let five_thirds = utonal_step(&third, &fifth);

    // assert
    assert_eq!(one, (1, 1));
    assert_eq!(five_thirds, (5, 3));
}

#[test]
fn it_is_a_prime_predicate() {
    // arrange
    let yes_1 = 7883;
    let yes_2 = 2;
    let yes_3 = 3;
    let no_1 = 1;
    let no_2 = 2000;
    let no_3 = 99;

    // assert
    assert_eq!(yes_1.is_prime(), true);
    assert_eq!(yes_2.is_prime(), true);
    assert_eq!(yes_3.is_prime(), true);
    assert_eq!(no_1.is_prime(), false);
    assert_eq!(no_2.is_prime(), false);
    assert_eq!(no_3.is_prime(), false);
}

#[test]
fn it_walks_otonally() {
    // arrange
    let fifth = Rational::from((3, 2));

    // act
    let result = fifth.walk(5);

    // assert
    assert_eq!(result, vec![(1, 1), (3, 2), (9, 8), (27, 16), (81, 64)]);
}

#[test]
fn it_walks_utonally() {
    // arrange
    let fourth = Rational::from((4, 3));

    // act
    let result = fourth.walk(5);

    // assert
    assert_eq!(result, vec![(1, 1), (4, 3), (16, 9), (32, 27), (128, 81)]);
}
