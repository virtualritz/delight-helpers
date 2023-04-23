use anyhow::{anyhow, Result};
use clap::CommandFactory;
use clap_complete::{
    generate,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};
use frame_sequence::parse_frame_sequence;
use human_panic::setup_panic;
use std::{path::Path, io};
use log::{error, info};
use fern::colors::{Color, ColoredLevelConfig};

const _VERSION: &str = env!("CARGO_PKG_VERSION");

mod rdl_cli;
use rdl_cli::*;

fn main() -> Result<()> {
    setup_panic!();

    run()
}

#[cfg(target_os = "windows")]
fn _sanitize_path_to_unc(path: &str) -> String {
    match path.chars().nth(1) {
        Some(':') => {
            "//?/".to_string() + &path.replace('\\', "/")
        }
        _ => path.to_string().replace('\\', "/")
    }
}

fn run() -> Result<()> {
    let cli = build_cli();

    // Get log level from cli.
    let log_level = match cli.verbose {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        #[cfg(debug_assertions)]
        3 => log::LevelFilter::Debug,
        #[cfg(debug_assertions)]
        _ => log::LevelFilter::Trace,
        #[cfg(not(debug_assertions))]
        _ => log::LevelFilter::Info,
    };

    let colors = ColoredLevelConfig::new().debug(Color::Magenta);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                // This will color the log level only, not the whole line. Just a touch.
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .apply()?;


    match cli.command {
        Command::Render(render_args) => render(render_args),
        Command::Cat(cat_args) => cat(cat_args),
        Command::GenerateCompletions { shell } => generate_completions(shell),
        _ => Ok(())
    }
}

fn generate_completions(shell: String) -> Result<()> {
    match shell.as_str() {
        "bash" => generate(Bash, &mut Cli::command(), "rdl", &mut io::stdout()),
        "elvish" => generate(Elvish, &mut Cli::command(), "rdl", &mut io::stdout()),
        "fish" => generate(Fish, &mut Cli::command(), "rdl", &mut io::stdout()),
        "powershell" => generate(PowerShell, &mut Cli::command(), "rdl", &mut io::stdout()),
        "zsh" => generate(Zsh, &mut Cli::command(), "rdl", &mut io::stdout()),
        _ => return Err(anyhow!("Unsupported shell '{shell}'")),
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

fn render(args: Render) -> Result<()> {
    let frame_sequence = if let Some(frame_sequence_string) = &args.frames {
        parse_frame_sequence(frame_sequence_string)
            .map_err(|e| anyhow!("Error in frame sequence expression{e}"))?
    } else {
        vec![]
    };

    for file_name in &args.file {
        let ctx = {
            let mut ctx_args = Vec::with_capacity(2);

            if args.cloud {
                ctx_args.push(nsi::integer!("cloud", true as _));
            } else if let Some(ref collective) = args.collective {
                ctx_args.push(nsi::string!("collective", collective.as_str()));
            }

            nsi::Context::new(Some(&ctx_args)).unwrap()
        };

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

                let file_name =
                    file_name.replace(frame_number_placeholder, &frame_string);

                render_file(&ctx, &file_name, &args);
            }
        } else {
            render_file(&ctx, file_name, &args);
        }

        if args.force_render {
            ctx.render_control(&[nsi::string!("action", "start")]);
            ctx.render_control(&[nsi::string!("action", "wait")]);
        }
    }

    Ok(())
}

fn render_file(ctx: &nsi::Context, file_name: &str, render: &Render) {
    if !Path::new(file_name).exists() {
        error!(
            "Skipping `{file_name}` because it does not exist"
        );
        return;
    }

    info!("Rendering '{file_name}'…");

    if !render.dry_run {
        nsi_render(ctx, file_name);
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

        let ctx = nsi::Context::new(Some(&args)).unwrap();

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
