use clap::{load_yaml, App, Shell};

static BINARY_NAME: &str = "rdl";

fn main() {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    let yaml = load_yaml!("src/cli.yml");
    let mut app = App::from_yaml(yaml);

    app.gen_completions(BINARY_NAME, Shell::Bash, &outdir);
    app.gen_completions(BINARY_NAME, Shell::Fish, &outdir);
    app.gen_completions(BINARY_NAME, Shell::Zsh, &outdir);
    app.gen_completions(BINARY_NAME, Shell::PowerShell, &outdir);
    app.gen_completions(BINARY_NAME, Shell::Elvish, &outdir);
}
