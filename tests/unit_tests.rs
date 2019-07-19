extern crate johnston;
extern crate rug;
use johnston::*;
use rug::Rational;
use std::f32;

#[test]
fn it_returns_factors() {
    // arrange
    let empty_vec: Vec<i32> = Vec::new();

    // assert
    assert_eq!(0.factors(), empty_vec);
    assert_eq!(6.factors(), vec![1, 2, 3, 6]);
    assert_eq!(7.factors(), vec![1, 7]);
    assert_eq!(34.factors(), vec![1, 2, 17, 34]);
}

#[test]
fn it_returns_the_greatest_prime_factor() {
    // assert
    assert_eq!(2.gpf(), 2);
    assert_eq!(15.gpf(), 5);
    assert_eq!(34.gpf(), 17);
    assert_eq!(49.gpf(), 7);
}

#[test]
fn it_is_a_power_of_two() {
    // assert
    assert_eq!(3.is_power_of_two(), false);
    assert_eq!(8.is_power_of_two(), true);
    assert_eq!(17.is_power_of_two(), false);
    assert_eq!(1024.is_power_of_two(), true);
    assert_eq!(1111.is_power_of_two(), false);
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
fn it_returns_the_correct_harmonic() {
    // arrange
    let tonic = Rational::from((1, 1));
    let fifth = Rational::from((3, 2));
    let third = Rational::from((5, 4));

    // act
    let a = tonic.harmonic(3);
    let b = third.harmonic(3);

    // assert
    assert_eq!(a, fifth);
    assert_eq!(b, Rational::from((15, 8)));
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
    let response = vec![
        Pitch::new(Rational::from((1, 1))),
        Pitch::new(Rational::from((3, 2))),
        Pitch::new(Rational::from((9, 8))),
        Pitch::new(Rational::from((27, 16))),
        Pitch::new(Rational::from((81, 64))),
    ];

    // act
    let result = fifth.walk(5);

    // assert
    assert_eq!(result, response);
}

#[test]
fn it_walks_utonally() {
    // arrange
    let p = |n: i32, d: i32| Pitch::new(Rational::from((n, d)));
    let fourth = Rational::from((4, 3));
    let response = vec![p(1, 1), p(4, 3), p(16, 9), p(32, 27), p(128, 81)];

    // act
    let result = fourth.walk(5);

    // assert
    assert_eq!(result, response);
}

#[test]
fn it_generates_a_lattice() {
    // arrange
    let p = |n: i32, d: i32| Pitch::new(Rational::from((n, d)));
    let args = &[3, 5];
    let steps = 3;
    let expected = vec![
        LatticeDimension {
            limit: args[0],
            otonal: vec![p(1, 1), p(3, 2), p(9, 8)],
            utonal: vec![p(1, 1), p(4, 3), p(16, 9)],
        },
        LatticeDimension {
            limit: args[1],
            otonal: vec![p(1, 1), p(5, 4), p(25, 16)],
            utonal: vec![p(1, 1), p(8, 5), p(32, 25)],
        },
    ];

    // act
    let result = gen_lattice(args, steps);

    // assert
    assert_eq!(result, expected);
}
