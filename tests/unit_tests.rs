extern crate johnston;
extern crate rug;
use johnston::*;
use rug::Rational;
use std::f32;

#[test]
fn it_returns_factors() {
    // arrange
    let six = 6;
    let seven = 7;
    let thirty_four = 34;

    // assert
    assert_eq!(factors(six), vec![1, 2, 3, 6]);
    assert_eq!(factors(seven), vec![1, 7]);
    assert_eq!(factors(thirty_four), vec![1, 2, 17, 34]);
}

#[test]
fn it_returns_the_greatest_prime_factor() {
    // arrange
    let fifteen = 15;
    let thirty_four = 34;
    let fourty_nine = 49;

    // assert
    assert_eq!(greatest_prime_factor(fifteen), 5);
    assert_eq!(greatest_prime_factor(thirty_four), 17);
    assert_eq!(greatest_prime_factor(fourty_nine), 7);
}

#[test]
fn it_return_a_list_from_a_ratio() {
    // arrange
    let a = Rational::from((3, 2));

    // assert
    assert_eq!(ratio_to_list(a), vec![3, 2]);
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
    assert_eq!(is_power_of_two(a), false);
    assert_eq!(is_power_of_two(b), true);
    assert_eq!(is_power_of_two(c), false);
    assert_eq!(is_power_of_two(d), true);
    assert_eq!(is_power_of_two(e), false);
}

#[test]
fn it_calculates_cents() {
    // Arrange.
    let fifth = Rational::from((3, 2));

    // Act.
    let a = calc_cents(&fifth);

    // Assert.
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
    assert_eq!(get_ordinal(&a), Ordinal::Otonal);
    assert_eq!(get_ordinal(&b), Ordinal::Otonal);
    assert_eq!(get_ordinal(&c), Ordinal::Utonal);
}

#[test]
fn it_gets_the_correct_limit() {
    // arrange
    let a = Rational::from((3, 2));
    let b = Rational::from((5, 4));
    let c = Rational::from((7, 5));
    let d = Rational::from((13, 8));

    // assert
    assert_eq!(get_limit(a), 3);
    assert_eq!(get_limit(b), 5);
    assert_eq!(get_limit(c), 7);
    assert_eq!(get_limit(d), 13);
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
    // Arrange.
    let a = Rational::from((18, 4));
    let b = Rational::from((9, 4));
    let c = Rational::from((6, 2));
    let d = Rational::from((3, 4));

    // Act.
    // Assert.
    assert_eq!(flatten_ratio(a), (9, 8));
    assert_eq!(flatten_ratio(b), (9, 8));
    assert_eq!(flatten_ratio(c), (3, 2));
    assert_eq!(flatten_ratio(d), (3, 2));
}

#[test]
fn it_steps_otonally() {
    // Arrange.
    let fifth = Rational::from((3, 2));
    let third = Rational::from((5, 4));

    // Act.
    let nine_eighths = otonal_step(&fifth, &fifth);
    let fifteen_eighths = otonal_step(&third, &fifth);

    // Assert.
    assert_eq!(nine_eighths, (9, 8));
    assert_eq!(fifteen_eighths, (15, 8));
}

#[test]
fn it_steps_utonally() {
    // Arrange.
    let fifth = Rational::from((3, 2));
    let third = Rational::from((5, 4));

    // Act.
    let one = utonal_step(&fifth, &fifth);
    let five_thirds = utonal_step(&third, &fifth);

    // Assert.
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
    assert_eq!(is_prime(yes_1), true);
    assert_eq!(is_prime(yes_2), true);
    assert_eq!(is_prime(yes_3), true);
    assert_eq!(is_prime(no_1), false);
    assert_eq!(is_prime(no_2), false);
    assert_eq!(is_prime(no_3), false);
}

#[test]
fn it_walks_otonally() {
    // Arrange.
    let fifth = Rational::from((3, 2));

    // Act.
    let result = walk(&fifth, 5);

    // Assert.
    assert_eq!(result, vec![(1, 1), (3, 2), (9, 8), (27, 16), (81, 64)]);
}

#[test]
fn it_walks_utonally() {
    // Arrange.
    let fourth = Rational::from((4, 3));

    // Act.
    let result = walk(&fourth, 5);

    // Assert.
    assert_eq!(result, vec![(1, 1), (4, 3), (16, 9), (32, 27), (128, 81)]);
}

