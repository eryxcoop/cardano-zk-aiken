use crate::parsers::int_parser;
use crate::token_zk::TokenZK as Token;
use crate::zk_examples::ZkExample;
use aiken_lang::ast::Span;
use aiken_lang::parser::{error::ParseError, extra::ModuleExtra};
use chumsky::prelude::*;
use ordinal::Ordinal;

pub struct LexInfo {
    pub tokens: Vec<(Token, Span)>,
    pub extra: ModuleExtra,
}

pub struct Lexer {
    lexer: Box<dyn Parser<char, Vec<(Token, Span)>, Error = ParseError>>,
}

impl Lexer {
    pub fn new() -> Self {
        let int = int_parser();

        let ordinal = text::int(10)
            .then_with(|index: String| {
                choice((just("st"), just("nd"), just("rd"), just("th")))
                    .map(move |suffix| (index.to_string(), suffix))
            })
            .validate(|(index, suffix), span, emit| match index.parse() {
                Err { .. } => {
                    emit(ParseError::invalid_tuple_index(span, index, None));
                    Token::Ordinal { index: 0 }
                }
                Ok(index) => {
                    let expected_suffix = Ordinal::<u32>(index).suffix();
                    if expected_suffix != suffix {
                        emit(ParseError::invalid_tuple_index(
                            span,
                            index.to_string(),
                            Some(expected_suffix.to_string()),
                        ))
                    }
                    Token::Ordinal { index }
                }
            });

        let op = choice((
            just("==").to(Token::EqualEqual),
            just('=').to(Token::Equal),
            just("..").to(Token::DotDot),
            just('.').to(Token::Dot),
            just("!=").to(Token::NotEqual),
            just('!').to(Token::Bang),
            just('?').to(Token::Question),
            just("<-").to(Token::LArrow),
            just("->").to(Token::RArrow),
            choice((
                just("<=").to(Token::LessEqual),
                just('<').to(Token::Less),
                just(">=").to(Token::GreaterEqual),
                just('>').to(Token::Greater),
            )),
            just('+').to(Token::Plus),
            just('-').to(Token::Minus),
            just('*').to(Token::Star),
            just('/').to(Token::Slash),
            just('%').to(Token::Percent),
            just("|>").to(Token::Pipe),
            just(',').to(Token::Comma),
            just(':').to(Token::Colon),
            just("||").to(Token::VbarVbar),
            just('|').to(Token::Vbar),
            just("&&").to(Token::AmperAmper),
            just('#').to(Token::Hash),
        ));

        let grouping = choice((
            just('(').to(Token::LeftParen),
            just(')').to(Token::RightParen),
            just('[').to(Token::LeftSquare),
            just(']').to(Token::RightSquare),
            just('{').to(Token::LeftBrace),
            just('}').to(Token::RightBrace),
        ));

        let escape = just('\\').ignore_then(
            just('\\')
                .or(just('"'))
                .or(just('n').to('\n'))
                .or(just('r').to('\r'))
                .or(just('t').to('\t'))
                .or(just('0').to('\0')),
        );

        let string = just('@')
            .ignore_then(just('"'))
            .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
            .then_ignore(just('"'))
            .collect::<String>()
            .map(|value| Token::String { value })
            .labelled("string");

        let bytestring = just('"')
            .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
            .then_ignore(just('"'))
            .collect::<String>()
            .map(|value| Token::ByteString { value })
            .labelled("bytestring");

        let keyword = text::ident().map(Self::keyword_mapping);

        fn comment_parser(token: Token) -> impl Parser<char, (Token, Span), Error = ParseError> {
            let n = match token {
                Token::ModuleComment => 4,
                Token::DocComment => 3,
                Token::Comment => 2,
                _ => unreachable!(),
            };

            choice((
                // NOTE: The first case here work around a bug introduced with chumsky=0.9.0 which
                // miscalculate the offset for empty comments.
                just("/".repeat(n))
                    .ignore_then(choice((text::newline().rewind(), end())))
                    .to(token.clone())
                    .map_with_span(move |token, span: Span| {
                        (token, Span::new((), span.start + n..span.end))
                    }),
                just("/".repeat(n)).ignore_then(
                    take_until(choice((text::newline().rewind(), end())))
                        .to(token)
                        .map_with_span(|token, span| (token, span)),
                ),
            ))
        }

        let newlines = choice((
            choice((just("\n\n"), just("\r\n\r\n"))).to(Token::EmptyLine),
            choice((just("\n"), just("\r\n"))).to(Token::NewLine),
        ));

        fn zk_parser() -> impl Parser<char, (Token, Span), Error = ParseError> {
            just("offchain")
                .ignore_then(just(' ').repeated().ignored())
                .ignore_then(ZkExample::parser())
                .map_with_span(|token, span| (token, span))
        }

        let lexer = choice((
            comment_parser(Token::ModuleComment),
            comment_parser(Token::DocComment),
            comment_parser(Token::Comment),
            zk_parser(),
            choice((
                ordinal, keyword, int, op, newlines, grouping, bytestring, string,
            ))
            .or(any().map(Token::Error).validate(|t, span, emit| {
                emit(ParseError::expected_input_found(
                    span,
                    None,
                    Some(t.clone()),
                ));
                t
            }))
            .map_with_span(|token, span| (token, span)),
        ))
        .padded_by(one_of(" \t").ignored().repeated())
        .recover_with(skip_then_retry_until([]))
        .repeated()
        .padded_by(one_of(" \t").ignored().repeated())
        .then_ignore(end());

        Self {
            lexer: Box::new(lexer),
        }
    }

    pub fn run(&mut self, src: &str) -> Result<LexInfo, Vec<ParseError>> {
        let len = src.len();

        let tokens = self.lexer.parse(chumsky::Stream::from_iter(
            Span::create(len, 1),
            src.chars().scan(0, |i, c| {
                let start = *i;
                let offset = c.len_utf8();
                *i = start + offset;
                Some((c, Span::create(start, offset)))
            }),
        ))?;

        let mut extra = ModuleExtra::new();

        let mut previous_is_newline = false;

        let tokens = tokens
            .into_iter()
            .filter_map(|(token, ref span)| {
                let current_is_newline = token == Token::NewLine || token == Token::EmptyLine;
                let result = match token {
                    Token::ModuleComment => {
                        extra.module_comments.push(*span);
                        None
                    }
                    Token::DocComment => {
                        extra.doc_comments.push(*span);
                        None
                    }
                    Token::Comment => {
                        extra.comments.push(*span);
                        None
                    }
                    Token::EmptyLine => {
                        extra.empty_lines.push(span.start);
                        None
                    }
                    Token::LeftParen => {
                        if previous_is_newline {
                            Some((Token::NewLineLeftParen, *span))
                        } else {
                            Some((Token::LeftParen, *span))
                        }
                    }
                    Token::Minus => {
                        if previous_is_newline {
                            Some((Token::NewLineMinus, *span))
                        } else {
                            Some((Token::Minus, *span))
                        }
                    }
                    Token::Pipe => {
                        if previous_is_newline {
                            Some((Token::NewLinePipe, *span))
                        } else {
                            Some((Token::Pipe, *span))
                        }
                    }
                    Token::NewLine => None,
                    _ => Some((token, *span)),
                };

                previous_is_newline = current_is_newline;

                result
            })
            .collect::<Vec<(Token, Span)>>();

        Ok(LexInfo { tokens, extra })
    }

    fn keyword_mapping(s: String) -> Token {
        match s.as_str() {
            "trace" => Token::Trace,
            // TODO: remove this in a future release
            "error" => Token::Fail,
            "fail" => Token::Fail,
            "once" => Token::Once,
            "as" => Token::As,
            "and" => Token::And,
            "or" => Token::Or,
            "expect" => Token::Expect,
            "const" => Token::Const,
            "fn" => Token::Fn,
            "test" => Token::Test,
            "if" => Token::If,
            "else" => Token::Else,
            "is" => Token::Is,
            "let" => Token::Let,
            "opaque" => Token::Opaque,
            "pub" => Token::Pub,
            "use" => Token::Use,
            "todo" => Token::Todo,
            "type" => Token::Type,
            "when" => Token::When,
            "validator" => Token::Validator,
            "via" => Token::Via,
            "bench" => Token::Benchmark,
            _ => {
                if s.chars().next().is_some_and(|c| c.is_uppercase()) {
                    Token::UpName {
                        // TODO: do not allow _ in upname
                        name: s,
                    }
                } else if s.starts_with('_') {
                    Token::DiscardName {
                        // TODO: do not allow uppercase letters in discard name
                        name: s,
                    }
                } else {
                    Token::Name {
                        // TODO: do not allow uppercase letters in name
                        name: s,
                    }
                }
            }
        }
    }
}
