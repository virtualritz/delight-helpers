use clap::{load_yaml, App};
use eyre::{eyre, Result};
use std::path::Path;

#[macro_use]
extern crate pest_derive;

mod frame_sequence_parser;
use frame_sequence_parser::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    run()
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
            println!("rdl version {}", VERSION);
            Ok(())
        }
        ("render", Some(render_args)) => render(render_args),
        ("cat", Some(cat_args)) => cat(cat_args),
        _ => Err(eyre!(
            "Unknown/missing subcommand. Please specify one of 'render', 'cat' or --help"
        )),
    }
}

fn nsi_render(ctx: &nsi::Context, file_name: &str) {
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

fn render(args: &clap::ArgMatches) -> Result<()> {
    let frame_sequence = if let Some(frame_sequence_string) = args.value_of("FRAMES") {
        parse_frame_sequence(frame_sequence_string)?
    } else {
        vec![]
    };

    match args.value_of("FILE") {
        Some(file_name) => {
            let ctx = if args.is_present("cloud") {
                nsi::Context::new(&[nsi::integer!("cloud", true as _)])
            } else {
                nsi::Context::new(&[])
            }
            .unwrap();

            if let Some(pos) = file_name.find('@') {
                if frame_sequence.is_empty() {
                    return Err(eyre!(
                        "[rdl] No frame sequence to fill placeholder `@` in `{}` specified.",
                        file_name
                    ));
                }

                let padding = if let Some(number) = file_name.get(pos + 1..pos + 2) {
                    number.parse::<usize>().unwrap_or(0)
                } else {
                    0
                };

                let frame_number_placeholder = if padding != 0 {
                    file_name.get(pos..pos + 2).unwrap()
                } else {
                    "@"
                };

                // Render frame sequence.
                for frame in frame_sequence {
                    let frame_string = if padding != 0 {
                        format!("{:0width$}", frame, width = padding)
                    } else {
                        format!("{}", frame)
                    };

                    let frame_file_name =
                        file_name.replace(frame_number_placeholder, &frame_string);

                    if args.is_present("verbose") || args.is_present("dry_run") {
                        println!("[rdl] Rendering `{}`.", frame_file_name);
                    }

                    if !args.is_present("dry_run") {
                        nsi_render(&ctx, &frame_file_name);
                    }
                }
            } else {
                if args.is_present("verbose") || args.is_present("dry_run") {
                    println!("[rdl] Rendering `{}`.", file_name);
                }

                if !args.is_present("dry_run") {
                    nsi_render(&ctx, file_name);
                }
            }
            Ok(())
        }

        None => Err(eyre!(
            "[rdl] render subcommand requires specifying a file to render"
        )),
    }
}

fn cat(cat_args: &clap::ArgMatches) -> Result<()> {
    if let Some(file_name) = cat_args.value_of("FILE") {
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
    Ok(())
}
