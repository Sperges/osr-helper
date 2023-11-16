use std::fmt::{Display, self};

use anyhow::Result;
use pest::{pratt_parser::PrattParser, Parser, iterators::Pairs};

use crate::roll::Roll;


#[derive(pest_derive::Parser)]
#[grammar = "dice.pest"]
pub struct DiceParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(comma, Left))
			.op(Op::infix(repeat, Left))
            .op(Op::prefix(collect))
            .op(Op::infix(multi_add, Left) | Op::infix(multi_subtract, Left) | Op::infix(multi_multiply, Left) | Op::infix(multi_divide, Left))
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(dice, Left) | Op::infix(keep_high, Left) | Op::infix(keep_low, Left) | Op::infix(drop_high, Left) | Op::infix(drop_low, Left))
            .op(Op::prefix(unary_minus))
    };
}

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    UnaryMinus(Box<Expr>),
    Collect(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Integer(i) => write!(f, "{}", i),
            Expr::UnaryMinus(expr) => write!(f, "{}", expr),
            Expr::Collect(expr) => write!(f, "{}", expr),
            Expr::BinOp { lhs, op, rhs } => write!(f, "{}{}{}", lhs, op, rhs),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    MultiAdd,
    MultiSubtract,
    MultiMultiply,
    MultiDivide,
    Dice,
    KeepHighest,
    KeepLowest,
    DropHighest,
    DropLowest,
    Comma,
	Repeat,
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Subtract => write!(f, "-"),
            Op::Multiply => write!(f, "*"),
            Op::Divide => write!(f, "/"),
            Op::MultiAdd => write!(f, "++"),
            Op::MultiSubtract => write!(f, "--"),
            Op::MultiMultiply => write!(f, "**"),
            Op::MultiDivide => write!(f, "//"),
            Op::Dice => write!(f, "d"),
            Op::KeepHighest => write!(f, "kh"),
            Op::KeepLowest => write!(f, "kl"),
            Op::DropHighest => write!(f, "dh"),
            Op::DropLowest => write!(f, "dl"),
            Op::Comma => write!(f, ","),
            Op::Repeat => write!(f, "@"),
        }
    }
}

#[derive(Debug)]
pub struct Dice {
    pub literal: String,
    pub expr: Expr,
}

impl Dice {
	pub fn _new(expr: Expr) -> Dice {
		Dice {
			literal: format!("{}", expr),
			expr,
		}
	}

    pub fn parse<'i>(expression: &'i str) -> Result<Dice> {
        let expr = Self::expr(&expression)?;
        Ok(Dice {
            literal: expression.to_string(),
            expr,
        })
    }

    fn expr<'i>(expression: &'i str) -> Result<Expr> {
        let pairs = DiceParser::parse(Rule::expression, expression)?
            .next()
            .expect(&format!("no pairs found in {}", expression))
            .into_inner();
        Ok(Self::parse_pairs(pairs))
    }

    fn parse_pairs(pairs: Pairs<Rule>) -> Expr {
        PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
                Rule::expr => Self::parse_pairs(primary.into_inner()),
                rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => Op::Add,
                    Rule::subtract => Op::Subtract,
                    Rule::multiply => Op::Multiply,
                    Rule::divide => Op::Divide,
                    Rule::multi_add => Op::MultiAdd,
                    Rule::multi_subtract => Op::MultiSubtract,
                    Rule::multi_multiply => Op::MultiMultiply,
                    Rule::multi_divide => Op::MultiDivide,
                    Rule::dice => Op::Dice,
                    Rule::keep_high => Op::KeepHighest,
                    Rule::keep_low => Op::KeepLowest,
                    Rule::drop_high => Op::DropHighest,
                    Rule::drop_low => Op::DropLowest,
                    Rule::comma => Op::Comma,
					Rule::repeat => Op::Repeat,
                    rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
                };
                Expr::BinOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                }
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
                Rule::collect => Expr::Collect(Box::new(rhs)),
                _ => unreachable!(),
            })
            .parse(pairs)
    }

    pub fn roll(&self) -> Roll {
        Roll::new(&self.expr)
    }
}