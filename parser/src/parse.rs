use combine::{ParseError, ParseResult, chainl1, many1, satisfy};
use combine::char::{alpha_num, letter, string};
use combine::primitives::{Parser, Stream};
use combine_language::{Identifier, LanguageDef, LanguageEnv};

use rsstreem::ast::*;

use std::ops::Deref;

type ParseFn<'a, I> = fn(&RsstreemEnv<'a, I>, I) -> ParseResult<Box<Ast>, I>;

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
    type Output = Box<Ast>;

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

    fn parse_integer(&self, input: I) -> ParseResult<Box<Ast>, I> {
        self.integer()
            .map(|x| Box::new(Ast::Int(x)))
            .parse_stream(input)
    }

    #[warn(dead_code)]
    pub fn integer64(&'b self) -> RsstreemParser<'a, 'b, I> {
        self.parser(RsstreemEnv::parse_integer)
    }

    fn parse_add(&self, input: I) -> ParseResult<Box<Ast>, I> {
        let plus_op = self.reserved_op("+")
            .map(|_| {
                move |lhs, rhs| {
                    Box::new(Ast::Op {
                                 op: Biop::Plus,
                                 lhs: lhs,
                                 rhs: rhs,
                             })
                }
            });

        chainl1(self.parser(RsstreemEnv::parse_integer), plus_op).parse_stream(input)
    }

    pub fn add(&'b self) -> RsstreemParser<'a, 'b, I> {
        self.parser(RsstreemEnv::parse_add)
    }
}

pub fn parse_code<'a>(code: &'a str) -> Result<Vec<Box<Ast>>, ParseError<&'a str>> {
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
        .with(many1::<Vec<Box<Ast>>, _>(env.add()))
        .parse(code)
        .map(|(e, _)| e)
}
