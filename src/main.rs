use clap::{load_yaml, App};
use itertools::Itertools;
use pest::Parser;
use std::{cmp::Ordering, collections::HashSet, io::Write, path::Path};

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "frame_format_grammar.pest"]
pub struct FrameSequenceParser;

#[macro_use]
extern crate error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
                        println!("{:?}", pair);
                        match pair.as_rule() {
                            Rule::PositiveNumber => {
                                let step = frame_to_number(pair);

                                match left.cmp(&right) {
                                    Ordering::Less => {
                                        (left..right).step_by(step as _).collect::<Vec<_>>()
                                    }
                                    Ordering::Greater => {
                                        let mut n = left;
                                        let mut result =
                                            Vec::with_capacity(((left - right) / step) as _);
                                        while n > right {
                                            result.push(n);
                                            n -= step;
                                        }
                                        result
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

fn main() {
    if let Err(ref e) = run() {
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }

    //println!("{:?}", binary_seq((0, 20)));
}

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).get_matches();

    // Read config file (if it exists).
    //let config_file = app.value_of("config").unwrap_or("rdla.toml");

    /*
    let mut config: Config = {
        if let Ok(mut file) = File::open(config_file) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            match toml::from_str::<Config>(&contents.as_str()) {
                Ok(toml) => toml,
                Err(e) => {
                    eprintln!("Config file error in '{}': {}.", config_file, e);
                    return Ok(());
                }
            }
        } else {
            // Set everything in Config to None.
            Default::default()
        }
    };*/

    match app.subcommand() {
        ("version", Some(_v)) => {
            println!("rdl version {}", VERSION)
        }
        ("render", Some(render_args)) => {
            //config.nsi_render.output.display = render_args.is_present("display");

            let frame_sequence = if let Some(frame_sequence_string) = render_args.value_of("FRAME")
            {
                match FrameSequenceParser::parse(Rule::FrameSequenceString, frame_sequence_string) {
                    Ok(token_tree) => {
                        let result =
                            remove_duplicates(frame_sequence_token_tree_to_frames(token_tree));
                        println!("{:?}", result);
                        result
                    }
                    Err(e) => {
                        println!("Error in frame sequence expression{}", e);
                        //return Ok(());
                        vec![]
                    }
                }
            } else {
                vec![]
            };

            if frame_sequence.is_empty() {
                println!("{:?}", frame_sequence);
            }

            match render_args.value_of("FILE") {
                Some(file_name) => {
                    let ctx = if render_args.is_present("cloud") {
                        nsi::Context::new(&[nsi::integer!("cloud", true as _)])
                    } else {
                        nsi::Context::new(&[])
                    }
                    .unwrap();

                    /*ctx.evaluate(&[
                        nsi::string!(
                            "type",
                            if file_name.len() > 3 && ".lua" == &file_name[file_name.len() - 4..] {
                                "lua"
                            } else {
                                "apistream"
                            }
                        ),
                        nsi::string!("filename", file_name),
                    ]);*/
                }
                //config.nsi_render.output.file_name = Some(file_name.to_string());
                None => eprintln!("[rdl] render subcommand requires specifying a file to render"),
            }
        }
        ("cat", Some(cat_args)) => {
            match cat_args.value_of("FILE") {
                Some(file_name) => {
                    let path = Path::new(cat_args.value_of("OUTPUT").unwrap_or("stdout"));

                    let mut args = vec![nsi::string!("streamfilename", path.to_str().unwrap())];

                    if cat_args.is_present("binary") {
                        args.push(nsi::string!("streamformat", "binarynsi"));
                    }

                    if cat_args.is_present("gzip") {
                        args.push(nsi::string!("streamcompression", "gzip"));
                    }

                    let mut expand = vec!["apistream"];
                    if cat_args.is_present("expand") {
                        expand.push("lua");
                        expand.push("dynamiclibrary");
                    }
                    args.push(nsi::strings!("executeprocedurals", &expand));

                    let ctx = nsi::Context::new(&args).unwrap();

                    ctx.evaluate(&[
                        nsi::string!(
                            "type",
                            if file_name.len() > 3 && ".lua" == &file_name[file_name.len() - 4..] {
                                "lua"
                            } else {
                                "apistream"
                            }
                        ),
                        nsi::string!("filename", file_name),
                    ]);
                }
                None => eprintln!("[rdl] cat subcommand requires specifying a FILE to dump"),
            }

            /*
            if "ply" == path.extension().unwrap() {
                model.write_ply(&path);
            } else {
                model.write_nsi(&path);
            }*/
        }
        /*("", None) => eprintln!(
            "No subcommand given. Please specify at least one of 'help, 'render' or 'cat'."
        ),*/
        _ => unreachable!(),
    }
    Ok(())
}
