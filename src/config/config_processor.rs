use std::collections::HashSet;

use super::KeyCombinations;
use super::parser::{Expressions, parse_expr};
use crate::key_buffer::{Event, KeyDeque};

fn process_config() {}

fn get_action<'a>(deq: &KeyDeque, combinations: &'a KeyCombinations) -> Option<&'a Expressions> {
    // return Some(&combinations[0].action);
    let mut event_in_buffer = Vec::<&Event>::with_capacity(deq.len());
    let mut s = HashSet::<&Event>::new();
    for e in deq.iter() {
        event_in_buffer.push(&e.event);
        s.insert(&e.event);
    }
    event_in_buffer.sort();

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parser() {
        process_config();
    }
}
