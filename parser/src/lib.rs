extern crate combine;
extern crate combine_language;
extern crate rsstreem;

pub mod parse;

#[cfg(test)]
mod tests {
    use super::*;
    use rsstreem::ast::*;

    #[test]
    fn add_expr_test() {
        let expr_string = "1 + 2";
        let correct_expr = Ast::Op {
            op: Biop::Plus,
            lhs: Box::new(Ast::Int(1)),
            rhs: Box::new(Ast::Int(2)),
        };

        assert_eq!(parse::parse_code(expr_string).unwrap()[0], correct_expr);
    }

    #[test]
    fn integer_test() {
        let expr_string = "100";
        let correct_expr = Ast::Int(100);

        assert_eq!(parse::parse_code(expr_string).unwrap()[0], correct_expr);
    }
}
