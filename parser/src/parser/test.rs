use std::{assert_eq, panic};

use crate::common::Group::{Brace, Bracket, Paren};
use crate::parser::Expression;

use super::expression::test::expression_strategy;
use super::parse;
use proptest::prelude::*;

#[test]
fn expression_with_explicit_params_neg() {
    let one: Expression = 1.into();
    let two: Expression = 2.into();
    {
        let input: Expression = -&one;
        let actual = input.with_explicit_parens();
        let expected = -(&one.in_parens());
        assert_eq!(
            actual, expected,
            r#"
            Actual:  {actual}
            Expected:{expected}
            "#
        )
    }
    {
        let input: Expression = -&one + -&two;
        let actual = input.with_explicit_parens();
        let expected = {
            let neg_one = -(&one.in_parens());
            let neg_two = -(&two.in_parens());

            neg_one.in_parens() + neg_two.in_parens()
        };
        assert_eq!(
            actual, expected,
            r#"
            Actual:  {actual}
            Expected:{expected}
            "#
        )
    }
}

#[test]
fn expression_with_explicit_params_smoke() {
    let one: Expression = 1.into();
    let two: Expression = 2.into();
    let three: Expression = 3.into();
    let four: Expression = 4.into();
    let input: Expression = (&-&one + &two).in_parens() * (&three - &four);
    let actual = input.with_explicit_parens();
    let expected = {
        let neg_one = -(&one.in_parens());
        let one_plus_two = &neg_one.in_parens() + &two.in_parens();
        let three_sub_four = &three.in_parens() - &four.in_parens();

        one_plus_two.in_parens().in_parens() * three_sub_four.in_parens()
    };
    assert_eq!(
        actual, expected,
        r#"
        Actual:  {actual}
        Expected:{expected}
        "#
    )
}

#[test]
fn expression_minimize_nesting_works() {
    let one: Expression = 1.into();
    let two: Expression = 2.into();
    let three: Expression = 3.into();
    let four: Expression = 4.into();
    let input = {
        let neg_one = -(&one.in_parens());
        let one_plus_two = &neg_one.in_parens() + &two.in_parens();
        let three_sub_four = &three.in_parens() - &four.in_parens();

        one_plus_two.in_parens().in_parens() * three_sub_four.in_parens()
    };
    let actual = input.minimize_nesting();
    let expected = (&-&one + &two).in_parens() * (&three - &four).in_parens();
    assert_eq!(
        actual, expected,
        r#"
        Actual:  {actual}
        Expected:{expected}
        "#
    )
}

#[test]
fn expression_minimize_nesting_collapses_mixed_groups() {
    let one: Expression = 1.into();
    let two: Expression = 2.into();
    let three: Expression = 3.into();
    {
        let input = one
            .bracket_with(Paren)
            .bracket_with(Paren)
            .bracket_with(Paren);
        let actual = input.minimize_nesting();
        let expected = one.clone();
        assert_eq!(
            actual, expected,
            r#"
            Actual:  {actual}
            Expected:{expected}
            "#
        )
    }
    {
        let input = one
            .bracket_with(Paren)
            .bracket_with(Bracket)
            .bracket_with(Paren);
        let actual = input.minimize_nesting();
        let expected = one.clone();
        assert_eq!(
            actual, expected,
            r#"
            Actual:  {actual}
            Expected:{expected}
            "#
        )
    }
    {
        let input = {
            let one_p = one.bracket_with(Paren);
            let two_p = two.bracket_with(Brace);
            let three_p = three.bracket_with(Bracket);

            ((one_p + two_p).bracket_with(Brace).bracket_with(Paren) + three_p)
                .bracket_with(Bracket)
        };
        let actual = input.minimize_nesting();
        let expected = (&(&one + &two).bracket_with(Brace) + &three).bracket_with(Bracket);
        assert_eq!(
            actual, expected,
            r#"
            Actual:  {actual}
            Expected:{expected}
            "#
        )
    }
}

proptest! {
    #[test]
    fn expression_parsing_works (expression in expression_strategy()) {
        let expected = expression.with_explicit_parens().minimize_nesting();
        let input = expected.unparse();
        let actual = match parse(&input) {
            Ok(exp) => exp,
            Err(error) => panic!("Input failed to parse: {error}"),
        };
        assert_eq!(
            actual,
            expected,
            r#"
            Actual:  {actual}
            Expected:{expected}
            "#
        )
    }
}
