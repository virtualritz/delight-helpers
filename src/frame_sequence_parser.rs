use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};
use eyre::{eyre, Result};
use pest::Parser;

#[derive(Parser)]
#[grammar = "frame_format_grammar.pest"]
pub struct FrameSequenceParser;

pub fn parse_frame_sequence(input: &str) -> Result<Vec<isize>> {

    match FrameSequenceParser::parse(Rule::FrameSequenceString, input) {
        Ok(token_tree) => {
            Ok(remove_duplicates(frame_sequence_token_tree_to_frames(token_tree)))
        }
        Err(e) => {
            Err(eyre!("Error in frame sequence expression{}", e))
        }
    }
}

fn chop(seq: &mut Vec<isize>, result: &mut Vec<isize>, elements: usize) {
    if seq.len() < elements {
        let mut new_seq = seq
            .iter()
            .tuple_windows()
            .flat_map(|pair: (&isize, &isize)| {
                let left = *pair.0;
                let right = (*pair.0 + *pair.1) / 2;
                if left < right {
                    result.push(right);
                    vec![left, right]
                } else {
                    vec![left]
                }
            })
            .collect::<Vec<_>>();
        new_seq.push(*seq.last().unwrap());
        if new_seq.len() < elements {
            chop(&mut new_seq, result, elements);
        }
        *seq = new_seq;
    }
}

fn binary_seq(range: (isize, isize)) -> Vec<isize> {
    match range.0.cmp(&range.1) {
        Ordering::Less => {
            let mut seq = vec![range.0, range.1];
            let mut result = seq.clone();
            chop(&mut seq, &mut result, (range.1 - range.0) as _);
            result
        }
        Ordering::Greater => {
            let mut seq = vec![range.1, range.0];
            let mut result = seq.clone();
            chop(&mut seq, &mut result, (range.0 - range.1) as _);
            result.reverse();
            result
        }
        Ordering::Equal => vec![range.0],
    }
}

fn frame_to_number(frame: pest::iterators::Pair<Rule>) -> isize {
    frame.as_str().parse::<isize>().unwrap()
}

fn frame_sequence_token_tree_to_frames(pairs: pest::iterators::Pairs<Rule>) -> Vec<isize> {
    pairs
        .into_iter()
        .flat_map(|pair| {
            match pair.as_rule() {
                Rule::FrameSequenceString | Rule::FrameSequence | Rule::FrameSequencePart => {
                    frame_sequence_token_tree_to_frames(pair.into_inner())
                }
                Rule::FrameRange => {
                    let mut pairs = pair.into_inner();
                    let left = frame_to_number(pairs.next().unwrap());
                    let right = frame_to_number(pairs.next().unwrap());

                    // Do we have an `@`?
                    if pairs.next().is_some() {
                        let pair = pairs.next().unwrap();
                        match pair.as_rule() {
                            Rule::PositiveNumber => {
                                let step = frame_to_number(pair);

                                match left.cmp(&right) {
                                    Ordering::Less => {
                                        (left..right + 1).step_by(step as _).collect::<Vec<_>>()
                                    }
                                    Ordering::Greater => {
                                        (right..left + 1).rev().step_by(step as _).collect::<Vec<_>>()
                                    }
                                    Ordering::Equal => vec![left],
                                }
                            }
                            Rule::BinarySequenceSymbol => binary_seq((left, right)),
                            _ => unreachable!(),
                        }
                    } else if left < right {
                        (left..right + 1).collect::<Vec<_>>()
                    } else if right < left {
                        (right..left + 1).rev().collect::<Vec<_>>()
                    }
                    // left == right
                    else {
                        vec![left]
                    }
                }
                Rule::Frame => vec![frame_to_number(pair)],
                _ => vec![],
            }
        })
        .collect::<Vec<_>>()
}

fn remove_duplicates(elements: Vec<isize>) -> Vec<isize> {
    let mut set = HashSet::<isize>::new();
    elements
        .iter()
        .filter_map(|e| {
            if set.contains(e) {
                None
            } else {
                set.insert(*e);
                Some(*e)
            }
        })
        .collect()
}