use std::sync::OnceLock;

use pest::pratt_parser::PrattParser;
use pest_derive::Parser;

#[derive(Parser, Clone, Copy)]
#[grammar = "grammar.pest"]
pub struct RParser;

fn pratt_parser() -> &'static PrattParser<Rule> {
    static PRATT: OnceLock<PrattParser<Rule>> = OnceLock::new();
    PRATT.get_or_init(|| {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // this is very unfinished, we first need a type to map parsed expressions into
        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
    })
}
