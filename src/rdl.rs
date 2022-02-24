use anyhow::{anyhow, Result};
use clap::CommandFactory;
use clap_complete::{
    generate,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};
use frame_sequence::parse_frame_sequence;
use std::{io, str::FromStr};
use wax::Glob;

const _VERSION: &str = env!("CARGO_PKG_VERSION");

mod rdl_cli;
use rdl_cli::*;

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    let cli = build_cli();

    // Read config file (if it exists).
    //let config_file = cli.value_of("config").unwrap_or("rdla.toml");

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

    match cli.command {
        Command::Render(render_args) => render(render_args),
        Command::Cat(cat_args) => cat(cat_args),
        Command::GenerateCompletions { shell } => generate_completions(shell),
    }
}

fn generate_completions(shell: String) -> Result<()> {
    match shell.as_str() {
        "bash" => generate(Bash, &mut Cli::command(), "rdl", &mut io::stdout()),
        "elvish" => generate(Elvish, &mut Cli::command(), "rdl", &mut io::stdout()),
        "fish" => generate(Fish, &mut Cli::command(), "rdl", &mut io::stdout()),
        "powershell" => generate(PowerShell, &mut Cli::command(), "rdl", &mut io::stdout()),
        "zsh" => generate(Zsh, &mut Cli::command(), "rdl", &mut io::stdout()),
        _ => return Err(anyhow!("Unknown shell '{shell}'.")),
    }

    Ok(())
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

fn render(render: Render) -> Result<()> {
    let frame_sequence = if let Some(frame_sequence_string) = &render.frames {
        parse_frame_sequence(frame_sequence_string)
            .map_err(|e| anyhow!("Error in frame sequence expression{e}"))?
    } else {
        vec![]
    };

    let cloud = render.cloud;
    for maybe_glob in &render.file {
        let glob = Glob::from_str(maybe_glob)?;

        for file_name in glob.walk(".", usize::MAX) {
            let file_name = file_name.map_err(|e| anyhow!("{e}"))?;
            let file_name = file_name.path().to_str().unwrap();

            let ctx = {
                if cloud {
                    nsi::Context::new(&[nsi::integer!("cloud", true as _)])
                } else {
                    nsi::Context::new(&[nsi::integer!("cloud", false as _)])
                }
            }
            .unwrap();

            if let Some(pos) = file_name.find('@') {
                if frame_sequence.is_empty() {
                    return Err(anyhow!(
                    "[rdl] No frame sequence to fill placeholder `@` in `{file_name}` specified.",
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
                for frame in &frame_sequence {
                    let frame_string = if padding != 0 {
                        format!("{:0width$}", frame, width = padding)
                    } else {
                        format!("{frame}")
                    };

                    let frame_file_name =
                        file_name.replace(frame_number_placeholder, &frame_string);

                    render_file(&ctx, &frame_file_name, &render);
                }
            } else {
                render_file(&ctx, &file_name, &render);
            }
        }
    }
    /*None => Err(eyre!(
        "[rdl] render subcommand requires specifying a file to render"
    )),*/
    Ok(())
}

fn render_file(ctx: &nsi::Context, file_name: &str, render: &Render) {
    if render.verbose || render.dry_run {
        println!("[rdl] Rendering `{}`.", file_name);
    }

    if !render.dry_run {
        nsi_render(&ctx, file_name);
    }
}

fn cat(cat: Cat) -> Result<()> {
    if let Some(file_name) = &cat.file {
        let path = cat.output.clone().unwrap_or("stdout".to_string());

        let mut args = vec![nsi::string!("streamfilename", path.as_str())];

        if cat.binary {
            args.push(nsi::string!("streamformat", "binarynsi"));
        }

        if cat.gzip {
            args.push(nsi::string!("streamcompression", "gzip"));
        }

        let mut expand = vec!["apistream"];
        if cat.expand {
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
            nsi::string!("filename", file_name.as_str()),
        ]);
    }
    Ok(())
}
