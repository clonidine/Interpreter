use petgraph::graph::{Graph, NodeIndex};

use crate::token::token::Token;

pub struct ParserTree {
    graph: Graph<Token, ()>,
    root: Option<NodeIndex>,
}

impl ParserTree {
    fn new() -> ParserTree {
        ParserTree {
            graph: Graph::new(),
            root: None,
        }
    }

    fn add_node(&mut self, token: Token) -> NodeIndex {
        self.graph.add_node(token)
    }

    fn add_edge(&mut self, parent: NodeIndex, child: NodeIndex) {
        self.graph.add_edge(parent, child, ());
    }

    fn set_root(&mut self, root: NodeIndex) {
        self.root = Some(root);
    }

    fn get_root(&self) -> Option<&Token> {
        self.root.map(|index| &self.graph[index])
    }

    fn build_parse_tree(tokens: Vec<Token>) -> Option<ParserTree> {
        // SOON

        None
    }
}
