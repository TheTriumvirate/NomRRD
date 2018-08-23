// helper functions
use nom::types::CompleteStr;
use datatypes::*;
use std::num::{ParseIntError, ParseFloatError};


pub fn from_int(input: CompleteStr) ->  Result<i32, ParseIntError> {
    (&input.to_string()).parse()
}

pub fn from_double(input: CompleteStr) -> Result<f64, ParseFloatError> {
    (&input.to_string()).parse()
}

pub fn from_complete_str(input: CompleteStr) -> Result<String, ()> {
    return Ok(input.to_string());
}

pub fn remove_comments(input: Vec<NRRDHeader>) -> Result<Vec<NRRDHeader>, ()> {
    return Ok(input.into_iter().filter(|h| !match h { NRRDHeader::Comment => true, _ => false, }).collect());
}

// TODO: RENAME TO SOMETHING REGARDING "part of integer" OR WHATEVER TO ENCOMPASS DASH
pub fn is_digit(c : char) -> bool {
    return c == '-' || c.is_digit(10);
}

// TODO: RENAME TO SOMETHING REGARDING "part of double" OR WHATEVER TO ENCOMPASS DASH
pub fn is_double_digit(c : char) -> bool {
    return c == '.' || c == '-' || c.is_digit(10);
}
