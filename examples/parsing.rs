use pest::Parser;
use rfmt::parser::*;

static EXAMPLE_CODE: &str = r#"
#' Example Function
#'
#' @param a A value
#' @param b A value
#' @param c A value
#'
#' @return A `character` string
#'
f <- function(a, b = 2, c = { expression }) {
    if (a + b) {
        "hello, world"  # return something
    } else {
        "fizzbuzz"      # or return something else
    }
}
"#;

fn main() {
    let pairs = RParser::parse(Rule::exprs, EXAMPLE_CODE);
    println!("{pairs:#?}")
}
