use std::{cmp::Ordering, fmt::Display};


use itertools::Itertools;

use rand::Rng;

use crate::dice::{Expr, Op};

#[derive(Debug)]
pub struct SubRoll {
    pub literal: String,
    pub values: Vec<i32>,
}

impl Display for SubRoll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} = {}",
            self.literal,
            self.values.iter().join(", "),
            self.values.iter().sum::<i32>(),
        )
    }
}

#[derive(Debug)]
pub struct Roll {
    pub sub_rolls: Vec<SubRoll>,
    pub values: Vec<i32>,
}

impl Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.values.iter().join(", "))
    }
}

impl Roll {
    pub fn new(expr: &Expr) -> Roll {
        match expr {
            Expr::Integer(i) => Roll {
                sub_rolls: Vec::new(),
                values: vec![*i],
            },
            Expr::UnaryMinus(rhs) => {
                let mut result = Self::new(rhs);
                result.values = vec![-result.sum()];
                result
            }
            Expr::Collect(rhs) => {
                let mut result = Self::new(rhs);
                result.values = vec![result.sum()];
                result
            }
            Expr::BinOp { lhs, op, rhs } => match op {
                Op::Add => Self::op(lhs, rhs, |x, y| x + y),
                Op::Subtract => Self::op(lhs, rhs, |x, y| x - y),
                Op::Multiply => Self::op(lhs, rhs, |x, y| x * y),
                Op::Divide => Self::op(lhs, rhs, |x, y| x / y),
                Op::MultiAdd => Self::multi_op(lhs, rhs, |x, y| x + y),
                Op::MultiSubtract => Self::multi_op(lhs, rhs, |x, y| x - y),
                Op::MultiMultiply => Self::multi_op(lhs, rhs, |x, y| x * y),
                Op::MultiDivide => Self::multi_op(lhs, rhs, |x, y| x / y),
                Op::Dice => Self::roll(lhs, rhs),
                Op::KeepHighest => Self::dice_op(lhs, rhs, Self::keep, |a, b| b.cmp(a)),
                Op::KeepLowest => Self::dice_op(lhs, rhs, Self::keep, |a, b| a.cmp(b)),
                Op::DropHighest => Self::dice_op(lhs, rhs, Self::drop, |a, b| b.cmp(a)),
                Op::DropLowest => Self::dice_op(lhs, rhs, Self::drop, |a, b| a.cmp(b)),
                Op::Comma => {
                    let mut left = Self::new(lhs);
                    let mut right = Self::new(rhs);
                    left.values = [left.values, right.values].concat();
                    left.sub_rolls.append(&mut right.sub_rolls);
                    left
                },
				Op::Repeat => {
					let mut left = Self::new(lhs);
					let mut values = Vec::new();
					let repeat = left.sum();
					for _ in 0..repeat.max(0) {
						let mut right = Self::new(rhs);
						left.sub_rolls.append(&mut right.sub_rolls);
						values.append(&mut right.values);
					}
					left.values = values;
					left
				},
            },
        }
    }

	fn roll(lhs: &Expr, rhs: &Expr) -> Roll {
		let mut left = Self::new(lhs);
		let mut right = Self::new(rhs);
		let dice = left.sum();
		let faces = right.sum();
		let values = {
			let mut values = Vec::new();
			let mut rng = rand::thread_rng();

			for _ in 0..dice.abs() {
				let value = rng.gen_range(1..=(faces.abs()));
				values.push(if dice.is_negative() ^ faces.is_negative() {
					-value
				} else {
					value
				});
			}

			values
		};
		left.values = values.clone();
		left.sub_rolls.append(&mut right.sub_rolls);
		left.sub_rolls.push(SubRoll {
			literal: format!("{}d{}", dice, faces),
			values: values.clone(),
		});
		left
	}

    fn dice_op<F, O>(lhs: &Expr, rhs: &Expr, o: O, f: F) -> Roll
    where
        F: FnMut(&i32, &i32) -> Ordering,
		O: Fn(Vec<i32>, i32, F) -> Vec<i32>
    {
        let mut left = Self::new(lhs);
        let mut right = Self::new(rhs);
        left.sub_rolls.append(&mut right.sub_rolls);
        left.values = o(left.values, right.sum(), f);
        left
    }

    fn keep<F>(values: Vec<i32>, k: i32, cmp: F) -> Vec<i32>
    where
        F: FnMut(&i32, &i32) -> Ordering,
    {
        if k.is_negative() {
            values
                .into_iter()
                .sorted_by(cmp)
                .rev()
                .take(k.abs() as usize)
                .collect::<Vec<i32>>()
        } else {
            values
                .into_iter()
                .sorted_by(cmp)
                .take(k.abs() as usize)
                .collect::<Vec<i32>>()
        }
    }

    fn drop<F>(values: Vec<i32>, k: i32, cmp: F) -> Vec<i32>
    where
        F: FnMut(&i32, &i32) -> Ordering,
    {
        if k.is_negative() {
            values
                .into_iter()
                .sorted_by(cmp)
                .rev()
                .skip(k.abs() as usize)
                .collect::<Vec<i32>>()
        } else {
            values
                .into_iter()
                .sorted_by(cmp)
                .skip(k.abs() as usize)
                .collect::<Vec<i32>>()
        }
    }

    fn op<F>(lhs: &Expr, rhs: &Expr, f: F) -> Roll
    where
        F: Fn(i32, i32) -> i32,
    {
        let mut left = Self::new(lhs);
        let mut right = Self::new(rhs);
        left.values = vec![f(left.sum(), right.sum())];
        left.sub_rolls.append(&mut right.sub_rolls);
        left
    }

    fn multi_op<F>(lhs: &Expr, rhs: &Expr, f: F) -> Roll
    where
        F: Fn(i32, i32) -> i32,
    {
        let mut left = Self::new(lhs);
        let mut values = Vec::new();
        for value in left.values.into_iter() {
            let mut right = Self::new(rhs);
            left.sub_rolls.append(&mut right.sub_rolls);
            values.push(f(value, right.sum()))
        }
        left.values = values;
        left
    }

    pub fn sum(&self) -> i32 {
        self.values.iter().sum::<i32>()
    }
}


