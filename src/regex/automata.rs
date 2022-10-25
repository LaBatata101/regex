use std::collections::{hash_map::Entry, BTreeMap, BTreeSet, HashMap};

use super::debug::AutomataDebug;
use super::helper::calculate_hash;
use super::parser::CharacterClassBinaryOp;
use super::parser::CharacterClassType;
use super::parser::{BinaryOp, RegexAST, UnaryOp};

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, Ord, PartialOrd)]
pub enum TransitionType {
    Epsilon,
    Symbol(char),
}

pub type State = usize;

#[derive(Debug, Clone)]
pub struct Dfa {
    start_state: State,
    final_states: BTreeSet<State>,
    transitions: BTreeMap<(State, TransitionType), State>,
}

impl Dfa {
    fn next_state(&self, state: State, transition: TransitionType) -> Option<State> {
        self.transitions.get(&(state, transition)).copied()
    }

    pub fn validate_str(&self, text: &str) -> bool {
        let mut state = Some(self.start_state);

        for char in text.chars() {
            if let Some(curr_state) = state {
                state = self.next_state(curr_state, TransitionType::Symbol(char));
            } else {
                break;
            }
        }

        match state {
            Some(state) => self.final_states.contains(&state),
            None => false,
        }
    }
}

impl AutomataDebug for Dfa {
    fn start_state(&self) -> State {
        self.start_state
    }

    fn final_states(&self) -> &BTreeSet<State> {
        &self.final_states
    }

    fn states(&self) -> BTreeSet<State> {
        self.transitions
            .iter()
            .map(|((state, _), _)| *state)
            .chain(self.transitions.values().copied())
            .collect()
    }

    fn transitions(&self) -> BTreeMap<(usize, TransitionType), BTreeSet<State>> {
        self.transitions
            .iter()
            .map(|(&state_transition, &dest_state)| (state_transition, BTreeSet::from([dest_state])))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Automata {
    start_state: State,
    final_states: BTreeSet<State>,
    transitions: BTreeMap<(State, TransitionType), BTreeSet<State>>,
}

impl Eq for Automata {}

impl PartialEq for Automata {
    fn eq(&self, other: &Self) -> bool {
        self.start_state == other.start_state
            && self.final_states == other.final_states
            && self.transitions == other.transitions
    }
}

impl AutomataDebug for Automata {
    fn start_state(&self) -> State {
        self.start_state
    }

    fn final_states(&self) -> &BTreeSet<State> {
        &self.final_states
    }

    fn states(&self) -> BTreeSet<State> {
        self.states()
    }

    fn transitions(&self) -> BTreeMap<(usize, TransitionType), BTreeSet<State>> {
        self.transitions.clone()
    }
}

impl Automata {
    pub fn new(start_state: State) -> Self {
        Self {
            start_state,
            final_states: BTreeSet::new(),
            transitions: BTreeMap::new(),
        }
    }

    pub fn eclosure(&self, states: BTreeSet<State>) -> BTreeSet<State> {
        let mut work: Vec<State> = Vec::from_iter(states.into_iter());
        let mut out: BTreeSet<State> = BTreeSet::new();

        while !work.is_empty() {
            let state = work.pop().unwrap();
            out.insert(state);

            if let Some(dest_states) = self.next_state(state, TransitionType::Epsilon) {
                for &dest_state in dest_states {
                    if out.insert(dest_state) {
                        work.push(dest_state);
                    }
                }
            }
        }

        out
    }

    fn delta(&self, states: &BTreeSet<State>, transition: TransitionType) -> BTreeSet<State> {
        states
            .iter()
            .filter_map(|state| self.next_state(*state, transition))
            .flatten()
            .copied()
            .collect()
    }

    fn merge_automata(&mut self, automata: Automata) {
        self.final_states.extend(automata.final_states);
        // TODO: check for a better way to merge two HashMaps
        for ((state, symbol), dest_states) in automata.transitions {
            self.transitions
                .entry((state, symbol))
                .or_insert_with(BTreeSet::new)
                .extend(dest_states);
        }
    }

    pub fn from_regex_expr(expr: RegexAST) -> Dfa {
        build_automata_from_ast(expr, &mut 0).convert_to_dfa()
    }

    /// Produce a minimized DFA using Brzozowski’s Algorithm.
    /// Reference: Engineering: A Compiler 2nd edition (Cooper, Keith D., Torczon, Linda),
    /// Chapter 2.6.2
    fn convert_to_dfa(self) -> Dfa {
        let nfa = reachable(subset(reverse(reachable(subset(reverse(self))))));

        Dfa {
            start_state: nfa.start_state(),
            transitions: nfa
                .transitions
                .iter()
                // .inspect(|x| println!("Transition: {x:?}"))
                .map(|(state_transition_types, dest_states)| {
                    // At this point the NFA only have one state in it's dest states
                    (*state_transition_types, dest_states.iter().next().copied().unwrap())
                })
                // .inspect(|x| println!("Transition after filter: {x:?}"))
                .collect(),
            final_states: nfa.final_states,
        }
    }

    pub fn states(&self) -> BTreeSet<State> {
        self.transitions
            .iter()
            .map(|((state, _), _)| *state)
            .chain(self.transitions.values().flatten().copied())
            .collect()
    }

    pub fn start_state(&self) -> State {
        self.start_state
    }

    pub fn final_states(&self) -> &BTreeSet<State> {
        &self.final_states
    }

    fn alphabet(&self) -> BTreeSet<char> {
        self.transitions
            .iter()
            .filter_map(|((_, transition_type), _)| match transition_type {
                TransitionType::Epsilon => None,
                TransitionType::Symbol(s) => Some(*s),
            })
            .collect()
    }

    fn next_state(&self, state: State, transition: TransitionType) -> Option<&BTreeSet<State>> {
        self.transitions.get(&(state, transition))
    }

    pub fn add_transition(&mut self, state: State, symbol: TransitionType, dest: State) {
        self.transitions
            .entry((state, symbol))
            .or_insert_with(BTreeSet::new)
            .insert(dest);
    }

    pub fn add_final_state(&mut self, state: State) {
        self.final_states.insert(state);
    }

    fn remove_state(&mut self, target: State) {
        self.transitions.retain(|&(state, _), _| state != target);
    }
}

pub fn build_automata_from_ast(tree: RegexAST, state: &mut State) -> Automata {
    let mut automata = Automata::new(*state);
    match tree {
        RegexAST::Binary(lhs, op, rhs) => {
            let mut lhs = build_automata_from_ast(*lhs, state);
            let rhs = build_automata_from_ast(*rhs, state);

            match op {
                BinaryOp::Union => {
                    automata.start_state = *state;
                    automata.add_transition(*state, TransitionType::Epsilon, lhs.start_state);
                    automata.add_transition(*state, TransitionType::Epsilon, rhs.start_state);

                    *state += 1;
                }
                BinaryOp::Concatenation => {
                    automata.start_state = lhs.start_state();
                    for fs in lhs.final_states() {
                        automata.add_transition(*fs, TransitionType::Epsilon, rhs.start_state);
                    }
                    lhs.final_states.clear();
                }
            }

            automata.merge_automata(lhs);
            automata.merge_automata(rhs);
        }
        RegexAST::Unary(lhs, op) => {
            let lhs = build_automata_from_ast(*lhs, state);

            for final_state in lhs.final_states() {
                automata.add_transition(*final_state, TransitionType::Epsilon, lhs.start_state);
            }

            match op {
                UnaryOp::ClosurePlus => {
                    let new_start = *state;
                    automata.add_transition(new_start, TransitionType::Epsilon, lhs.start_state);
                    automata.start_state = new_start;
                }
                UnaryOp::ClosureStar => {
                    let new_start = *state;
                    automata.add_transition(new_start, TransitionType::Epsilon, lhs.start_state);
                    automata.start_state = new_start;
                    automata.add_final_state(new_start);
                }
            }

            *state += 1;

            automata.merge_automata(lhs);
        }
        RegexAST::Symbol(symbol) => return create_automata_for_transtition_type(TransitionType::Symbol(symbol), state),
        RegexAST::CharacterClass(character_class_type) => return parse_character_class(character_class_type, state),
        RegexAST::EmptyString => return create_automata_for_transtition_type(TransitionType::Epsilon, state),
    }

    automata
}

fn parse_character_class(char_class_type: CharacterClassType, state: &mut usize) -> Automata {
    let mut automata = Automata::new(*state);
    match char_class_type {
        CharacterClassType::Single(symbol) => {
            return create_automata_for_transtition_type(TransitionType::Symbol(symbol), state);
        }
        CharacterClassType::Binary(lhs, CharacterClassBinaryOp::Union, rhs) => {
            let lhs = parse_character_class(*lhs, state);
            let rhs = parse_character_class(*rhs, state);

            automata.start_state = *state;
            automata.add_transition(*state, TransitionType::Epsilon, lhs.start_state);
            automata.add_transition(*state, TransitionType::Epsilon, rhs.start_state);

            automata.merge_automata(lhs);
            automata.merge_automata(rhs);

            *state += 1;
        }
        CharacterClassType::Binary(lhs, CharacterClassBinaryOp::Range, rhs) => {
            let lhs = if let CharacterClassType::Single(lhs) = *lhs {
                lhs
            } else {
                panic!("Wrong type for lhs in range")
            };
            let rhs = if let CharacterClassType::Single(rhs) = *rhs {
                rhs
            } else {
                panic!("Wrong type for rhs in range")
            };

            automata.start_state = *state;
            *state += 1;
            let final_state = *state;
            *state += 1;
            automata.add_final_state(final_state);
            for symbol in lhs..=rhs {
                automata.add_transition(automata.start_state(), TransitionType::Symbol(symbol), final_state);
            }
        }
    }

    automata
}

fn create_automata_for_transtition_type(transition: TransitionType, state: &mut State) -> Automata {
    let mut automata = Automata::new(*state);
    automata.add_transition(*state, transition, *state + 1);
    automata.add_final_state(*state + 1);

    *state += 2;

    automata
}

pub fn reachable(mut automata: Automata) -> Automata {
    let alphabet = automata.alphabet();
    let mut reachable_states = BTreeSet::from([automata.start_state()]);
    let mut new_states = BTreeSet::from([automata.start_state()]);

    while !new_states.is_empty() {
        let temp: BTreeSet<State> = alphabet
            .iter()
            .flat_map(|symbol| automata.delta(&new_states, TransitionType::Symbol(*symbol)))
            .collect();

        new_states = &temp - &reachable_states;
        reachable_states = &reachable_states | &new_states;
    }

    let unreachable_states = &automata.states() - &reachable_states;
    for unreachable_state in unreachable_states {
        automata.remove_state(unreachable_state);
    }

    automata
}

pub fn reverse(automata: Automata) -> Automata {
    let mut new_automata = Automata::new(automata.states().iter().max().copied().unwrap_or_default() + 1);
    new_automata.add_final_state(automata.start_state);

    for ((orig_state, symbol), dest_states) in automata.transitions {
        for dest_state in dest_states {
            new_automata.add_transition(dest_state, symbol, orig_state);
        }
    }

    for final_state in &automata.final_states {
        new_automata.add_transition(new_automata.start_state, TransitionType::Epsilon, *final_state);
    }

    new_automata
}

pub fn subset(automata: Automata) -> Automata {
    let alphabet = automata.alphabet();
    let dest_states = automata.eclosure(BTreeSet::from([automata.start_state()]));

    let mut work_list = Vec::new();

    let mut curr_state = 0;
    let mut dest_state = 0;
    let mut new_automata = Automata::new(curr_state);
    let mut new_states: HashMap<u64, State> = HashMap::new();

    new_states.insert(calculate_hash(&dest_states), curr_state);
    work_list.push(dest_states);

    while !work_list.is_empty() {
        let states = work_list.pop().unwrap();
        if let Some(state) = new_states.get(&calculate_hash(&states)) {
            curr_state = *state;
        }

        if !(&automata.final_states & &states).is_empty() {
            new_automata.add_final_state(curr_state);
        }

        for symbol in &alphabet {
            let subset = automata.eclosure(automata.delta(&states, TransitionType::Symbol(*symbol)));
            let subset_hash = calculate_hash(&subset);
            dest_state = if let Some(&dest_state) = new_states.get(&subset_hash) {
                dest_state
            } else {
                // TODO: find a better way to create state labels that doesnt exist
                loop {
                    dest_state += 1;
                    if !new_states.values().any(|&state| state == dest_state) {
                        break;
                    }
                }
                dest_state
            };

            new_automata.add_transition(curr_state, TransitionType::Symbol(*symbol), dest_state);

            if let Entry::Vacant(new_states) = new_states.entry(subset_hash) {
                new_states.insert(dest_state);
                work_list.push(subset);
            }
        }
    }

    new_automata
}
