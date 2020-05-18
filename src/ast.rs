trait Node {
    fn token_literal();
}

trait Statement: Node {
    fn statement_node();
}

trait Expression: Node {
    fn expression_node();
}

struct Program {
   Statements : Box<dyn Statement>,
}

impl Program {
    pub fn token_literal() {
        if p.Statemens.len()
    }
}
