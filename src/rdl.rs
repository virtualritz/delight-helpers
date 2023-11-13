use anyhow::{anyhow, Result};
use clap::CommandFactory;
use clap_complete::{
    generate,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};
use fern::colors::{Color, ColoredLevelConfig};
use human_panic::setup_panic;
use std::io;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod glibc {
    include!(concat!(env!("OUT_DIR"), "/glibc_version.rs"));
}

mod rdl_cli;
use rdl_cli::*;

mod watch;
use watch::*;

mod render;
use render::*;

fn main() -> Result<()> {
    setup_panic!();

    run()
}

#[cfg(target_os = "windows")]
fn _sanitize_path_to_unc(path: &str) -> String {
    match path.chars().nth(1) {
        Some(':') => "//?/".to_string() + &path.replace('\\', "/"),
        _ => path.to_string().replace('\\', "/"),
    }
}

fn run() -> Result<()> {
    let cli = build_cli();

    // Setup logging ==========================================================

    // Get log level from cli.
    let log_level = match cli.verbose {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        #[cfg(debug_assertions)]
        2 => log::LevelFilter::Debug,
        #[cfg(debug_assertions)]
        _ => log::LevelFilter::Trace,
        #[cfg(not(debug_assertions))]
        _ => log::LevelFilter::Error,
    };

    let colors = ColoredLevelConfig::new().debug(Color::Magenta);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}",
                format!(
                    "{: <16} [rdl] {}",
                    // This will color the log level only, not the whole line. Just a touch.
                    format!("[{}]", colors.color(record.level())),
                    //record.target(),
                    message
                )
            ))
        })
        .level(log_level)
        .apply()?;

    #[cfg(target_os = "windows")]
    rlimit::setmaxstdio(8192).unwrap_or_else(|| warning!("Could not set maximum of open files"));

    // Execute subcommand =====================================================
    match cli.command {
        Command::Render(args) => render(args),
        Command::Cat(args) => cat(args),
        Command::Watch(args) => watch(args),
        Command::Version => version(),
        Command::GenerateCompletions { shell } => generate_completions(shell),
    }
}

fn generate_completions(shell: String) -> Result<()> {
    match shell.as_str() {
        "bash" => generate(Bash, &mut Cli::command(), "rdl", &mut io::stdout()),
        "elvish" => generate(Elvish, &mut Cli::command(), "rdl", &mut io::stdout()),
        #[cfg(feature = "fig")]
        "fig" => clap_complete::generate(
            clap_complete_fig::Fig,
            &mut Cli::command(),
            "rdl",
            &mut io::stdout(),
        ),
        "fish" => generate(Fish, &mut Cli::command(), "rdl", &mut io::stdout()),
        #[cfg(feature = "nushell")]
        "nushell" => clap_complete::generate(
            clap_complete_nushell::Nushell,
            &mut Cli::command(),
            "rdl",
            &mut io::stdout(),
        ),
        "powershell" => generate(PowerShell, &mut Cli::command(), "rdl", &mut io::stdout()),
        "zsh" => generate(Zsh, &mut Cli::command(), "rdl", &mut io::stdout()),
        _ => return Err(anyhow!("Unsupported shell '{shell}'")),
    }

    Ok(())
}

fn version() -> Result<()> {
    eprintln!(
        "rdl {}\n\
        compiled with {}",
        //env!("CARGO_PACKAGE_VERSION"),
        built_info::PKG_VERSION,
        built_info::RUSTC_VERSION,
    );

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    {
        let glibc = glibc_version::get_version().map_err(|e| anyhow!("{e}"))?;

        eprintln!(
            "built against {} and running with {}.{}",
            glibc::GLIBC_VERSION,
            glibc.major,
            glibc.minor
        );
    }

    eprintln!(
        "using lib3delight {}\n\
        lib3delight is {}",
        delight::version(),
        delight::copyright(),
    );

    Ok(())
}

fn cat(args: Cat) -> Result<()> {
    if let Some(file_name) = &args.file {
        let path = args.output.clone().unwrap_or_else(|| "stdout".to_string());

        let mut ctx_args = vec![nsi::string!("streamfilename", path.as_str())];

        if args.binary {
            ctx_args.push(nsi::string!("streamformat", "binarynsi"));
        }

        if args.gzip {
            ctx_args.push(nsi::string!("streamcompression", "gzip"));
        }

        let mut expand = vec!["apistream"];

        if args.expand {
            expand.push("dynamiclibrary");
            expand.push("RiProcDynamicLoad");
        }

        ctx_args.push(nsi::strings!("executeprocedurals", &expand));

        let ctx = nsi::Context::new(Some(&ctx_args)).unwrap();

        ctx.evaluate(&[
            nsi::string!(
                "type",
                if file_name.len() > 3 && ".lua" == &file_name[file_name.len() - 4..] {
                    "lua"
                } else {
                    "apistream"
                }
            ),
            nsi::integer!("nostream", true as _),
            nsi::string!("filename", file_name.as_str()),
        ]);
    }

    Ok(())
}
