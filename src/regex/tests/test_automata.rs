use std::collections::BTreeSet;

use crate::regex::{automata::*, parser::parse_regex};

fn create_automata() -> Automata {
    let mut auto = Automata::new(0);
    auto.add_transition(0, TransitionType::Epsilon, 1);
    auto.add_transition(0, TransitionType::Epsilon, 5);
    auto.add_transition(0, TransitionType::Epsilon, 8);
    auto.add_transition(1, TransitionType::Symbol('a'), 2);
    auto.add_transition(2, TransitionType::Symbol('b'), 3);
    auto.add_transition(3, TransitionType::Symbol('c'), 4);
    auto.add_transition(5, TransitionType::Symbol('b'), 6);
    auto.add_transition(6, TransitionType::Symbol('c'), 7);
    auto.add_transition(8, TransitionType::Symbol('a'), 9);
    auto.add_transition(9, TransitionType::Symbol('d'), 10);
    auto.add_final_state(4);
    auto.add_final_state(7);
    auto.add_final_state(10);

    auto
}

#[test]
fn test_reverse_automata() {
    let reversed_automata = reverse(create_automata());
    let mut expected_automata = Automata::new(11);

    expected_automata.add_final_state(0);
    expected_automata.add_transition(1, TransitionType::Epsilon, 0);
    expected_automata.add_transition(2, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(3, TransitionType::Symbol('b'), 2);
    expected_automata.add_transition(4, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(5, TransitionType::Epsilon, 0);
    expected_automata.add_transition(6, TransitionType::Symbol('b'), 5);
    expected_automata.add_transition(8, TransitionType::Epsilon, 0);
    expected_automata.add_transition(7, TransitionType::Symbol('c'), 6);
    expected_automata.add_transition(9, TransitionType::Symbol('a'), 8);
    expected_automata.add_transition(10, TransitionType::Symbol('d'), 9);
    expected_automata.add_transition(11, TransitionType::Epsilon, 4);
    expected_automata.add_transition(11, TransitionType::Epsilon, 7);
    expected_automata.add_transition(11, TransitionType::Epsilon, 10);

    assert_eq!(reversed_automata, expected_automata)
}

#[test]
fn test_subset_reverse_automata() {
    let subset_reversed_automata = subset(reverse(create_automata()));
    let mut expected_automata = Automata::new(0);
    expected_automata.add_final_state(4);
    expected_automata.add_final_state(5);
    expected_automata.add_final_state(6);

    expected_automata.add_transition(0, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('c'), 2);
    expected_automata.add_transition(0, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(2, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(2, TransitionType::Symbol('b'), 5);
    expected_automata.add_transition(2, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(2, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(3, TransitionType::Symbol('a'), 4);
    expected_automata.add_transition(3, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(3, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(3, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(4, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(4, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(4, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(4, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(5, TransitionType::Symbol('a'), 6);
    expected_automata.add_transition(5, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(5, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(5, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(6, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(6, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(6, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(6, TransitionType::Symbol('d'), 1);

    assert_eq!(subset_reversed_automata, expected_automata)
}

#[test]
fn test_reachable_subset_reverse_automata() {
    let subset_reversed_automata = reachable(subset(reverse(create_automata())));
    let mut expected_automata = Automata::new(0);
    expected_automata.add_final_state(4);
    expected_automata.add_final_state(5);
    expected_automata.add_final_state(6);

    expected_automata.add_transition(0, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('c'), 2);
    expected_automata.add_transition(0, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(2, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(2, TransitionType::Symbol('b'), 5);
    expected_automata.add_transition(2, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(2, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(3, TransitionType::Symbol('a'), 4);
    expected_automata.add_transition(3, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(3, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(3, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(4, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(4, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(4, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(4, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(5, TransitionType::Symbol('a'), 6);
    expected_automata.add_transition(5, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(5, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(5, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(6, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(6, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(6, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(6, TransitionType::Symbol('d'), 1);

    assert_eq!(subset_reversed_automata, expected_automata)
}

#[test]
fn test_reverse_reachable_subset_reverse_automata() {
    let reverse_subset_reversed_automata = reverse(reachable(subset(reverse(create_automata()))));
    let mut expected_automata = Automata::new(7);
    expected_automata.add_final_state(0);

    expected_automata.add_transition(1, TransitionType::Symbol('a'), 0);
    expected_automata.add_transition(1, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('a'), 2);
    expected_automata.add_transition(1, TransitionType::Symbol('a'), 4);
    expected_automata.add_transition(1, TransitionType::Symbol('a'), 6);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 0);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 4);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 5);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 6);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 2);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 4);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 5);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 6);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 2);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 4);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 5);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 6);
    expected_automata.add_transition(2, TransitionType::Symbol('c'), 0);
    expected_automata.add_transition(3, TransitionType::Symbol('d'), 0);
    expected_automata.add_transition(4, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(5, TransitionType::Symbol('b'), 2);
    expected_automata.add_transition(6, TransitionType::Symbol('a'), 5);
    expected_automata.add_transition(7, TransitionType::Epsilon, 4);
    expected_automata.add_transition(7, TransitionType::Epsilon, 5);
    expected_automata.add_transition(7, TransitionType::Epsilon, 6);

    assert_eq!(reverse_subset_reversed_automata, expected_automata)
}

#[test]
fn test_subset_reverse_subset_reversed_automata() {
    let subset_reverse_subset_reversed_automata = subset(reverse(reachable(subset(reverse(create_automata())))));
    let mut expected_automata = Automata::new(0);
    expected_automata.add_final_state(4);

    expected_automata.add_transition(0, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('b'), 2);
    expected_automata.add_transition(0, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(0, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 2);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 4);
    expected_automata.add_transition(2, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(2, TransitionType::Symbol('b'), 3);
    expected_automata.add_transition(2, TransitionType::Symbol('c'), 4);
    expected_automata.add_transition(2, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(3, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(3, TransitionType::Symbol('b'), 3);
    expected_automata.add_transition(3, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(3, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(4, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(4, TransitionType::Symbol('b'), 3);
    expected_automata.add_transition(4, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(4, TransitionType::Symbol('d'), 3);

    assert_eq!(subset_reverse_subset_reversed_automata, expected_automata)
}

#[test]
fn test_reachable_subset_reverse_subset_reversed_automata() {
    let reachable_subset_reverse_subset_reversed_automata =
        reachable(subset(reverse(reachable(subset(reverse(create_automata()))))));
    let mut expected_automata = Automata::new(0);
    expected_automata.add_final_state(4);

    expected_automata.add_transition(0, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('b'), 2);
    expected_automata.add_transition(0, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(0, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('b'), 2);
    expected_automata.add_transition(1, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(1, TransitionType::Symbol('d'), 4);
    expected_automata.add_transition(2, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(2, TransitionType::Symbol('b'), 3);
    expected_automata.add_transition(2, TransitionType::Symbol('c'), 4);
    expected_automata.add_transition(2, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(3, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(3, TransitionType::Symbol('b'), 3);
    expected_automata.add_transition(3, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(3, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(4, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(4, TransitionType::Symbol('b'), 3);
    expected_automata.add_transition(4, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(4, TransitionType::Symbol('d'), 3);

    assert_eq!(reachable_subset_reverse_subset_reversed_automata, expected_automata)
}

#[test]
fn create_automata_from_regex_character_class_range() {
    let automata = build_automata_from_ast(parse_regex("[a-e]"), &mut 0);
    let mut expected_automata = Automata::new(0);
    expected_automata.add_transition(0, TransitionType::Symbol('a'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('b'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('c'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('d'), 1);
    expected_automata.add_transition(0, TransitionType::Symbol('e'), 1);
    expected_automata.add_final_state(1);

    assert_eq!(automata, expected_automata)
}

#[test]
fn create_automata_from_regex_character_class_range2() {
    let automata = build_automata_from_ast(parse_regex("1[a-e]"), &mut 0);
    let mut expected_automata = Automata::new(0);
    expected_automata.add_transition(0, TransitionType::Symbol('1'), 1);
    expected_automata.add_transition(1, TransitionType::Epsilon, 2);
    expected_automata.add_transition(2, TransitionType::Symbol('a'), 3);
    expected_automata.add_transition(2, TransitionType::Symbol('b'), 3);
    expected_automata.add_transition(2, TransitionType::Symbol('c'), 3);
    expected_automata.add_transition(2, TransitionType::Symbol('d'), 3);
    expected_automata.add_transition(2, TransitionType::Symbol('e'), 3);
    expected_automata.add_final_state(3);

    assert_eq!(automata, expected_automata)
}

#[test]
fn test_eclosure() {
    let mut automata = Automata::new(0);
    automata.add_transition(0, TransitionType::Epsilon, 1);
    automata.add_transition(0, TransitionType::Epsilon, 2);
    automata.add_transition(1, TransitionType::Epsilon, 3);
    automata.add_transition(2, TransitionType::Epsilon, 4);
    automata.add_transition(4, TransitionType::Symbol('a'), 5);
    automata.add_transition(5, TransitionType::Symbol('b'), 6);
    automata.add_final_state(5);
    automata.add_final_state(6);

    assert_eq!(automata.eclosure(BTreeSet::from([0])), BTreeSet::from([0, 1, 2, 3, 4]));
}
