#[warn(dead_code)] // Delete it later

use combine::{ParseError, ParseResult};
use combine::char::{alpha_num, letter, string};
use combine::combinator::*;
use combine::primitives::{Parser, Stream};
use combine_language::{Identifier, LanguageDef, LanguageEnv};

use rsstreem::ast::*;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;

type ParseFn<'a, I> = fn(&RsstreemEnv<'a, I>, I) -> ParseResult<Ast, I>;

struct RsstreemParser<'a: 'b, 'b, I>
    where I: Stream<Item = char> + 'b
{
    env: &'b RsstreemEnv<'a, I>,
    parser: ParseFn<'a, I>,
}

impl<'a, 'b, I> Parser for RsstreemParser<'a, 'b, I>
    where I: Stream<Item = char> + 'b
{
    type Input = I;
    type Output = Ast;

    fn parse_stream(&mut self, input: I) -> ParseResult<Self::Output, Self::Input> {
        (self.parser)(self.env, input)
    }
}

struct RsstreemEnv<'a, I>
    where I: Stream<Item = char>
{
    env: LanguageEnv<'a, I>,
}

impl<'a, I> Deref for RsstreemEnv<'a, I>
    where I: Stream<Item = char>
{
    type Target = LanguageEnv<'a, I>;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl<'a, 'b, I> RsstreemEnv<'a, I>
    where I: Stream<Item = char>,
          I::Range: 'b
{
    fn parser(&'b self, parser: ParseFn<'a, I>) -> RsstreemParser<'a, 'b, I> {
        RsstreemParser {
            env: self,
            parser: parser,
        }
    }

    // ----- test code here
    fn parse_integer(&self, input: I) -> ParseResult<Ast, I> {
        self.integer().map(|x| Ast::Int(x)).parse_stream(input)
    }

    #[warn(dead_code)]
    pub fn integer64(&'b self) -> RsstreemParser<'a, 'b, I> {
        self.parser(RsstreemEnv::parse_integer)
    }

    fn parse_add(&self, input: I) -> ParseResult<Ast, I> {
        let plus_op = self.reserved_op("+")
            .map(|_| {
                move |lhs, rhs| {
                    Ast::Op {
                        op: Biop::Plus,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
            });

        chainl1(self.parser(RsstreemEnv::parse_integer), plus_op).parse_stream(input)
    }

    pub fn add(&'b self) -> RsstreemParser<'a, 'b, I> {
        self.parser(RsstreemEnv::parse_add)
    }
    // ----- /test code here

    fn parse_program(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_topstmts(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_topstmt_list(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_topstmt(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_stmts(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_stmt_list(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_stmt(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }

    fn parse_var(&self, input: I) -> ParseResult<Ast, I> {
        self.identifier().map(|v| Ast::Var(v)).parse_stream(input)
    }

    fn parse_fname(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_expr(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_condition(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_opt_else(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_opt_args(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_arg(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_args(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_primary(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_opt_block(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_pterm(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_pary(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_pstruct(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }
    fn parse_pattern(&self, input: I) -> ParseResult<Ast, I> {
        unimplemented!()
    }

    fn parse_cparam(&self, input: I) -> ParseResult<Ast, I> {
        let op_lambda = self.reserved_op("->")
            .map(|_| {
                     Ast::Plambda {
                         pat: Box::new(Ast::Null),
                         cond: Box::new(Ast::Null),
                         body: Box::new(Ast::Null),
                         next: RefCell::new(vec![]),
                     }
                 });


        let if_pattern = self.reserved("if")
            .with(self.parser(RsstreemEnv::parse_expr))
            .skip(self.reserved("->"))
            .map(|c| {
                     Ast::Plambda {
                         pat: Box::new(Ast::Null),
                         cond: Box::new(c),
                         body: Box::new(Ast::Null),
                         next: RefCell::new(vec![]),
                     }
                 });

        op_lambda.or(if_pattern).parse_stream(input)
    }

    fn parse_case_body(&self, input: I) -> ParseResult<Ast, I> {
        let top_case_stmt = self.reserved("case")
            .with(self.parser(RsstreemEnv::parse_cparam))
            .and(self.parser(RsstreemEnv::parse_stmts))
            .map(|(p, s)| match p {
                     Ast::Plambda {
                         pat,
                         cond,
                         body,
                         next,
                     } => {
                         Ast::Plambda {
                             pat: pat,
                             cond: cond,
                             body: Box::new(s),
                             next: next,
                         }
                     }
                     _ => panic!("not match case_body"),
                 });

        let seaq_case_stmt = self.parser(RsstreemEnv::parse_case_body)
            .skip(self.reserved("case"))
            .and(self.parser(RsstreemEnv::parse_cparam))
            .and(self.parser(RsstreemEnv::parse_stmts))
            .map(|((b, p), s)| match p {
                     Ast::Plambda {
                         pat: p_pat,
                         cond: p_cond,
                         body: p_body,
                         next: p_next,
                     } => {
                         let boxied_cparam = Box::new(Ast::Plambda {
                                                          pat: p_pat,
                                                          cond: p_cond,
                                                          body: Box::new(s),
                                                          next: p_next,
                                                      });
                         match b {
                             Ast::Plambda {
                                 pat,
                                 cond,
                                 body,
                                 next,
                             } => {
                                 next.borrow_mut().push(boxied_cparam);

                                 Ast::Plambda {
                                     pat: pat,
                                     cond: cond,
                                     body: body,
                                     next: next,
                                 }
                             }
                             _ => panic!("not match case_body"),
                         }
                     }

                     _ => panic!("not match case_params"),
                 });

        top_case_stmt.or(seaq_case_stmt).parse_stream(input)
    }

    fn parse_block(&self, input: I) -> ParseResult<Ast, I> {
        let normal_blk = self.parser(RsstreemEnv::parse_stmts)
            .map(|s| {
                     Ast::Lambda {
                         args: Box::new(Ast::Null),
                         body: Box::new(s),
                         block: 1,
                     }
                 });

        let blk_with_param = self.parser(RsstreemEnv::parse_bparam)
            .and(self.parser(RsstreemEnv::parse_stmts))
            .map(|(b, s)| {
                     Ast::Lambda {
                         args: Box::new(b),
                         body: Box::new(s),
                         block: 0,
                     }
                 });

        let case_blk = self.parser(RsstreemEnv::parse_case_body);

        let case_else_blk = self.parser(RsstreemEnv::parse_case_body)
            .skip(self.reserved("else"))
            .skip(self.reserved_op("->"))
            .and(self.parser(RsstreemEnv::parse_stmts))
            .map(|(c, s)| {
                     Ast::Plambda {
                         pat: Box::new(Ast::Null),
                         cond: Box::new(Ast::Null),
                         body: Box::new(s),
                         next: RefCell::new(vec![Box::new(c)]),
                     }
                 });

        // FIXME: Occur type error when use `choice` or `choice!`
        self.braces(normal_blk.or(blk_with_param).or(case_blk).or(case_else_blk))
            .parse_stream(input)
    }

    fn parse_bparam(&self, input: I) -> ParseResult<Ast, I> {
        self.reserved_op("->")
            .map(|_| Ast::Null)
            .or(self.parser(RsstreemEnv::parse_f_args)
                    .skip(self.reserved_op("->")))
            .parse_stream(input)
    }

    fn parse_opt_f_args(&self, input: I) -> ParseResult<Ast, I> {
        sep_by::<Vec<Box<Ast>>, _, _>(self.parser(RsstreemEnv::parse_var).map(|v| Box::new(v)),
                                      token(','))
                .map(|v| Ast::Args { data: v })
                .parse_stream(input)
    }

    fn parse_f_args(&self, input: I) -> ParseResult<Ast, I> {
        sep_by1::<Vec<Box<Ast>>, _, _>(self.parser(RsstreemEnv::parse_var).map(|v| Box::new(v)),
                                       token(','))
                .map(|v| Ast::Args { data: v })
                .parse_stream(input)
    }

    fn parse_opt_terms(&self, input: I) -> ParseResult<(), I> {
        skip_many(token(';').or(token('\n'))).parse_stream(input)
    }

    fn parse_terms(&self, input: I) -> ParseResult<(), I> {
        skip_many1(token(';').or(token('\n'))).parse_stream(input)
    }

    // fn parse_term(&self, input: I) -> ParseResult<(), I> {
    //     skip_many().parse_stream(input)
    // }
}

pub fn parse_code<'a>(code: &'a str) -> Result<Vec<Ast>, ParseError<&'a str>> {
    let streem_env =
        LanguageEnv::new(LanguageDef {
                             ident: Identifier {
                                 start: letter(),
                                 rest: alpha_num(),
                                 reserved: ["if",
                                            "else",
                                            "skip",
                                            "case",
                                            "emif",
                                            "return",
                                            "namespace",
                                            "class",
                                            "import",
                                            "def",
                                            "method",
                                            "new",
                                            "nil",
                                            "true",
                                            "false"]
                                         .iter()
                                         .map(|x| (*x).into())
                                         .collect(),
                             },
                             op: Identifier {
                                 start: satisfy(|c| "+-*/%=!<>&|)(:".chars().any(|x| x == c)),
                                 rest: satisfy(|c| "+-*/%=!<>&|)(:".chars().any(|x| x == c)),
                                 reserved: ["::", ":", "==", "=", "(", ")", "+", "->", "*", "/",
                                            "%", "!=", "<=", ">=", "<-", "->", ">", "<", "&&", "&"]
                                         .iter()
                                         .map(|x| (*x).into())
                                         .collect(),
                             },
                             comment_start: string("/*").map(|_| ()),
                             comment_end: string("*/").map(|_| ()),
                             comment_line: string("//").map(|_| ()),
                         });

    let env = RsstreemEnv { env: streem_env };

    env.white_space()
        .with(many1::<Vec<Ast>, _>(env.add()))
        .parse(code)
        .map(|(e, _)| e)
}
