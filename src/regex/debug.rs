use std::collections::{BTreeMap, BTreeSet};

use graphviz_rust::{
    cmd::{CommandArg, Format},
    printer::PrinterContext,
};

use super::automata::{State, TransitionType};

type Node<'a> = (State, &'a str);
type Edge<'a> = (Node<'a>, String, Node<'a>);

pub trait AutomataDebug {
    fn start_state(&self) -> State;
    fn final_states(&self) -> &BTreeSet<State>;
    fn states(&self) -> BTreeSet<State>;
    fn transitions(&self) -> BTreeMap<(usize, TransitionType), BTreeSet<State>>;
}

pub struct AutomataPrinter {
    nodes: Vec<(usize, String)>,
    edges: Vec<(usize, String, usize)>,
    start_state: State,
    final_states: BTreeSet<State>,
}

impl AutomataPrinter {
    pub fn new(nfa: &impl AutomataDebug) -> Self {
        let nodes = nfa
            .states()
            .iter()
            .map(|state| (*state, format!("q{}", state)))
            .collect();

        let edges: Vec<(usize, TransitionType, usize)> = nfa
            .transitions()
            .iter()
            .flat_map(|((state, transition_type), dest_states)| {
                dest_states
                    .iter()
                    .map(|dest_state| (*state, *transition_type, *dest_state))
            })
            .collect();

        let mut duplicates: BTreeMap<(usize, usize), Vec<String>> = BTreeMap::new();
        for (state, t, dest_state) in edges {
            duplicates
                .entry((state, dest_state))
                .or_insert_with(Vec::new)
                .push(match t {
                    TransitionType::Epsilon => String::from("&epsilon;"),
                    TransitionType::Symbol(symbol) => symbol.to_string(),
                });
        }

        let edges: Vec<(usize, String, usize)> = duplicates
            .into_iter()
            .map(|((state, dest_state), symbols)| (state, symbols.join(", "), dest_state))
            .collect();

        Self {
            nodes,
            edges,
            start_state: nfa.start_state(),
            final_states: nfa.final_states().clone(),
        }
    }

    pub fn save_to_file(&self, filename: &str) {
        let mut buffer: Vec<u8> = Vec::new();

        dot::render(self, &mut buffer).unwrap();

        let graph = graphviz_rust::parse(String::from_utf8(buffer).unwrap().as_str()).unwrap();
        graphviz_rust::exec(
            graph,
            &mut PrinterContext::default(),
            vec![
                CommandArg::Format(Format::Svg),
                CommandArg::Output(filename.to_string()),
            ],
        )
        .unwrap();
    }
}

impl<'a> dot::Labeller<'a, Node<'a>, Edge<'a>> for AutomataPrinter {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("AUTOMATON").unwrap()
    }

    fn node_id(&'a self, n: &Node<'a>) -> dot::Id<'a> {
        dot::Id::new(format!("Q{}", n.0)).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Node<'b>) -> dot::LabelText<'b> {
        let &(_, label) = n;
        dot::LabelText::LabelStr(label.into())
    }

    fn node_shape(&'a self, node: &Node<'a>) -> Option<dot::LabelText<'a>> {
        if self.final_states.contains(&node.0) {
            Some(dot::LabelText::LabelStr("doublecircle".into()))
        } else {
            None
        }
    }

    fn node_color(&'a self, node: &Node<'a>) -> Option<dot::LabelText<'a>> {
        if node.0 == self.start_state {
            Some(dot::LabelText::LabelStr("green".into()))
        } else {
            None
        }
    }

    fn edge_label<'b>(&'b self, edge: &Edge<'b>) -> dot::LabelText<'b> {
        let (_, transition_type, _) = edge;
        dot::LabelText::LabelStr(transition_type.clone().into())
    }
}

impl<'a> dot::GraphWalk<'a, Node<'a>, Edge<'a>> for AutomataPrinter {
    fn nodes(&'a self) -> dot::Nodes<'a, Node<'a>> {
        self.nodes
            .iter()
            .map(|(state, state_lbl)| (*state, state_lbl.as_str()))
            .collect()
    }

    fn edges(&'a self) -> dot::Edges<'a, Edge<'a>> {
        self.edges
            .iter()
            .map(|(state, transition_type, dest_state)| {
                (
                    (*state, self.nodes[*state % self.nodes.len()].1.as_str()),
                    transition_type.clone(),
                    (*dest_state, self.nodes[*dest_state % self.nodes.len()].1.as_str()),
                )
            })
            .collect()
    }

    fn source(&self, e: &Edge<'a>) -> Node<'a> {
        e.0
    }

    fn target(&self, e: &Edge<'a>) -> Node<'a> {
        e.2
    }
}
