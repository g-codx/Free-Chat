use std::collections::HashSet;
use std::num::ParseIntError;

pub fn split_as_i64_set(value: String, separator: char) -> Result<HashSet<i64>, ParseIntError> {
    value.split(separator).map(|str| str.parse()).collect()
}

pub fn split_as_i64_vec(value: String, separator: char) -> Result<Vec<i64>, ParseIntError> {
    value.split(separator).map(|str| str.parse()).collect()
}
