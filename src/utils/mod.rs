use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, Div};

pub use std::fs;
pub use std::collections::{HashMap, HashSet};
pub use num::integer::lcm;
pub use winnow::Parser;
pub use colored::Colorize;

pub use itertools::Itertools;

pub mod gvec2;
pub mod vec2;
pub mod vec2_128;
pub mod vec3;
pub mod vec3_128;
pub mod solution_import;

pub mod parse {
    pub use winnow::prelude::*;
    pub use winnow::token::*;
    pub use winnow::combinator::*;
    pub use winnow::stream::AsChar;


    pub fn id(input: &mut &str) -> PResult<String> {
        take_while(1.., AsChar::is_alphanum).parse_next(input).and_then(|s| Ok(s.to_owned()))
    }

    pub fn int(input: &mut &str) -> PResult<i64> {
        take_while(1.., AsChar::is_dec_digit).parse_next(input).and_then(|s| Ok(s.parse::<i64>().unwrap()))
    }
    
    pub fn alphanums(mut input: &str) -> Vec<String> {
        if let Ok((_prefix, list, _postfix)) = (
            take_while(0.., |c: char| !c.is_alphanum()),
            separated(
                1..,
                take_while(1.., AsChar::is_alphanum),
                take_while(1.., |c: char| !c.is_alphanum())
            ),
            take_while(0.., |c: char| !c.is_alphanum()),
        )
        .parse_next(&mut input) as PResult<(&str, Vec<&str>, &str)> {
            list.iter().map(|&s| s.to_owned()).collect::<Vec<String>>()
        } else {
            panic!("failed to parse alphanums")
        }
    }
}

use std::{fmt::Debug, str::FromStr};

pub trait IterHelpers<T> {
    fn vec(self) -> Vec<T>;
    fn pair(self) -> (T, T);
}

impl<IterT, T> IterHelpers<T> for IterT
    where IterT: Iterator<Item=T> {
    fn vec(self) -> Vec<T> {
        self.collect::<Vec<T>>()
    }
    fn pair(mut self) -> (T, T) {
        (self.next().unwrap(), self.next().unwrap())
    }
}

pub trait PairHelpers<T> {
    fn map_pair<F, R>(&self, map: F) -> (R,R)
    where F: FnMut(&T) -> R;
}

impl<T> PairHelpers<T> for (T,T) {
    fn map_pair<F, R>(&self, mut map: F) -> (R, R) where F: FnMut(&T) -> R {
        (map(&self.0), map(&self.1))
    }
}

pub trait UnwrapIterResult<T> {
    fn unwrap(self) -> impl Iterator<Item=T>;
}

impl<IterT, T, E> UnwrapIterResult<T> for IterT
    where IterT: Iterator<Item=Result<T,E>>, E: Debug {
    fn unwrap(self) -> impl Iterator<Item=T> {
      self.map(|r| r.unwrap())
    }
}

pub trait UnwrapIterOption<T> {
    fn unwrap(self) -> impl Iterator<Item=T>;
}

impl<IterT, T> UnwrapIterOption<T> for IterT
    where IterT: Iterator<Item=Option<T>> {
    fn unwrap(self) -> impl Iterator<Item=T> {
      self.map(|r| r.unwrap())
    }
}

pub trait ParseStrIter {
    fn parse_i64(self) -> impl Iterator<Item=i64>;
}

impl<'a, IterT> ParseStrIter for IterT
    where IterT: Iterator<Item=&'a str>
{
    fn parse_i64(self) -> impl Iterator<Item=i64> {
        self.map(|s| s.parse::<i64>().unwrap())
    }
}

pub trait VecHelper<T> {
    fn map<B, F>(&self, fmap: F) -> impl Iterator<Item = B> 
        where F: FnMut(&T) -> B;
/*
    fn filter<F>(&self, pred: F) -> impl Iterator<Item = T>
        where F: FnMut(&T) -> bool;
*/
    fn any<F>(&self, pred: F) -> bool
        where F: FnMut(&T) -> bool;

    fn pair(self) -> (T, T);
}

impl<T> VecHelper<T> for Vec<T> {
    fn map<B, F>(&self, fmap: F) -> impl Iterator<Item = B> 
        where F: FnMut(&T) -> B
    {
        self.iter().map(fmap)
    }
/*
    fn filter<F>(&self, pred: F) -> impl Iterator<Item=&T>
        where F: FnMut(&T) -> bool
    {
        self.iter().filter(|&v| pred(v))
    }
*/
    fn any<F>(&self, pred: F) -> bool where F: FnMut(&T) -> bool {
        self.iter().any(pred)
    }

    fn pair(mut self) -> (T, T) {
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        (a, b)
    }
}

pub trait NumHelper<NumT> {
    fn sum(&self) -> NumT;
}

impl NumHelper<i64> for Vec<i64> {
    fn sum(&self) -> i64 {
        self.iter().copied().reduce(|l, r| l + r).unwrap()
    }
}

pub type I = i64;
