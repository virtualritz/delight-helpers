use clap::{Parser, Subcommand};

#[inline]
pub fn build_cli() -> Cli {
    Cli::parse()
}

#[derive(Parser)]
#[command(
    name = "rdl",
    about = "Renders or filters NSI streams or Lua NSI files with 3Delight",
    after_long_help = "",
    max_term_width = 80
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Render(Render),
    Cat(Cat),
    #[command(
        name = "generate-completions",
        about = "Generate completion scripts for various shells",
        display_order = 9999
    )]
    GenerateCompletions {
        #[arg(
            help = "The shell to generate completions for",
            value_parser = clap::builder::PossibleValuesParser::new(["bash", "fish", "zsh", "powershell", "elvish"])
        )]
        shell: String,
    },
}

#[derive(Parser)]
#[command(
    arg_required_else_help = true,
    about = "Render an image of result with 3Delight",
//    help_message = "Print this/a long help message."
)]
pub struct Render {
    #[arg(
        name = "FILE",
        index = 1,
        help = "The NSI FILE(s) to render",
        long_help = "The NSI FILE(s) to render\n\
            Frame number placeholders are specified using @[padding]:\n\
            foo.@.nsi  ⟶   foo.1.nsi, foo.2.nsi, …\n\
            foo.@4.nsi ⟶   foo.0001.nsi, foo.0002.nsi, …\n\n\
            Globbing using \"<pattern>\" (in quotes) is supported -\n\
            even if your shell has no support for it:\n\
            \"**/{*.{nsi,lua}}\" ⟶   all .nsi and .lua files in the\n\
                                   current folder and its subfolders\n"
    )]
    pub file: Vec<String>,

    #[arg(
        long,
        short = 'C',
        conflicts_with = "cloud",
        help = "Render using the the given 3Delight COLLECTIVE"
    )]
    pub collective: Option<String>,

    #[arg(
        long,
        short = 'c',
        conflicts_with = "collective",
        help = "Render using 3Delight Cloud"
    )]
    pub cloud: bool,

    #[arg(
        long,
        short = 'd',
        help = "Send the image(s) being rendered to 3Delight Display",
        long_help = "Send 3Delight Display (a copy of) the image(s) being\n\
                rendered.\n"
    )]
    pub display: bool,

    #[arg(long, short = 't', help = "Launch the render using number of THREADS")]
    pub threads: Option<usize>,

    #[arg(
        long,
        short = 'v',
        help = "Print the names of the file(s) being rendered"
    )]
    pub verbose: bool,

    #[arg(long, short = 'p', help = "Print rendering progress at each bucket")]
    pub progress: bool,

    #[arg(
        long,
        help = "Do not render, just print what would be done",
        long_help = "Do not render, just print the name of the file(s) to be\n\
                rendered.\n"
    )]
    pub dry_run: bool,

    #[arg(
        long,
        short = 'f',
        help = "FRAME(s) to render – 1,2,10-20,40-30@2",
        long_help = "FRAME(S) to render\n\
            They can be specified individually:\n\
            1,2​,3,5,8,13\n\
            Or as a squence:\n\
            10-15   ⟶   10, 11, 12, 13, 14, 15\n\
            With an optional step size:\n\
            10-20@2 ⟶   10, 12, 14, 16, 18, 20\n\
            Step size is always positive.\n\
            To render a sequence backwards specify the range in reverse:\n\
            42-33@3 ⟶   42, 39, 36, 33\n\
            With binary splitting. Useful to quickly check if a sequence\n\
            has ‘issues’ in some frames:\n\
            10-20@b ⟶   10, 20, 15, 12, 17, 11, 13, 16, 18, 14, 19\n\
            The last frame of a sequence will be omitted if\n\
            the specified step size does not touch it:\n\
            80-70@4 ⟶   80, 76, 72\n"
    )]
    pub frames: Option<String>,
    //short = 'I'

    //ignore_glob <pattern>
}

#[derive(Parser)]
#[command(
    arg_required_else_help = true,
    about = "Dump the input as an NSI stream to stdout or a file",
//    help_message = "Print this help message."
)]
pub struct Cat {
    #[arg(long, short = 'b', help = "Encode NSI stream in binary format")]
    pub binary: bool,

    #[arg(long, short = 'g', help = "Compress NSI stream using GNU zip format")]
    pub gzip: bool,

    #[arg(long, short = 'e', help = "Expand archives and procedurals")]
    pub expand: bool,

    #[arg(long = "expand-archives", help = "Expand archives")]
    pub expand_archives: bool,

    #[arg(long = "expand-procedurals", help = "Expand procedurals")]
    pub expand_procedurals: bool,

    #[arg(name = "FILE", help = "The NSI FILE(s) to dump")]
    pub file: Option<String>,

    #[arg(
        long,
        short = 'o',
        help = "Dump NSI stream to OUTPUT",
        long_help = "Dump NSI stream to OUTPUT instead of stdout"
    )]
    pub output: Option<String>,
}
