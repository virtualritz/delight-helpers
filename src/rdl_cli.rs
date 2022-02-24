use clap::{Parser, Subcommand};

#[inline]
pub fn build_cli() -> Cli {
    Cli::parse()
}

#[derive(Parser)]
#[clap(
    name = "rdl",
    about = "Renders or filters NSI streams or Lua NSI files",
    after_help = "‘rdl -h’ prints a brief overview while ‘rdl --help’ gives all details",
    after_long_help = "",
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "Display the renderer version")]
    Render(Render),
    Cat(Cat),
    #[clap(
        name = "generate-completions",
        about = "Generate completion scripts for various shells",
        display_order = 9999,
    )]
    GenerateCompletions {
        #[clap(
            help = "The shell to generate completions for",
            possible_values = &["bash", "fish", "zsh", "powershell", "elvish"]
        )]
        shell: String,
    },
}

#[derive(Parser)]
#[clap(
    arg_required_else_help = true,
    about = "Render an image of result with 3Delight",
//    help_message = "Print this/a long help message."
)]
pub struct Render {
    #[clap(
        long,
        short = 'u',
        conflicts_with = "cloud",
        help = "Render `locally`, in the `cloud` or in the given collective",
        long_help = "Render using the local machine (default):\n\
                --using local\n\
                Render using the cloud:\n\
                --using cloud\n\
                Render using the colletive `Molodchy`:\n\
                --using Molodchy\n"
    )]
    pub using: Option<String>,

    #[clap(
        long,
        short = 'c',
        conflicts_with = "using",
        help = "Use 3Delight Cloud to render the specified file(s)"
    )]
    pub cloud: bool,

    #[clap(
        long,
        short = 'd',
        help = "Send the image(s) being rendered to 3Delight Display",
        long_help = "Send 3Delight Display (a copy of) the image(s) being\n\
                rendered.\n"
    )]
    pub display: bool,

    #[clap(long, short = 't', help = "Launch the render using number of THREADS")]
    pub threads: Option<usize>,

    #[clap(
        long,
        short = 'v',
        help = "Print the names of the file(s) being rendered"
    )]
    pub verbose: bool,

    #[clap(long, short = 'p', help = "Print rendering progress at each bucket")]
    pub progress: bool,

    #[clap(
        long,
        help = "Do not render, just print what would be done",
        long_help = "Do not render, just print the name of the file(s) to be\n\
                rendered.\n"
    )]
    pub dry_run: bool,

    #[clap(
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
    #[clap(
        name = "FILE",
        index = 1,
        help = "The NSI FILE(s) to render",
        long_help = "The NSI FILE(s) to render\n\
            Frame number placeholders are specified using @[padding]:\n\
            foo.@.nsi  ⟶   foo.1.nsi, foo.2.nsi, …\n\
            foo.@4.nsi ⟶   foo.0001.nsi, foo.0002.nsi, …\n\n\
            Globbing using \"<pattern>\" (in quotes) is supported -\n\
            even if your shell has support for it:\n\
            \"**/{*.{nsi,lua}}\" ⟶ all .nsi and .lua files in the\n\
                                   current folder and its subfolders\n"
    )]
    pub file: Vec<String>,
}

#[derive(Parser)]
#[clap(
    arg_required_else_help = true,
    about = "Dump the input as an NSI stream to stdout/a file",
//    help_message = "Print this help message."
)]
pub struct Cat {
    #[clap(long, short = 'b', help = "Encode NSI stream in binary format")]
    pub binary: bool,

    #[clap(long, short = 'g', help = "Compress NSI stream using GNU zip format")]
    pub gzip: bool,

    #[clap(long, short = 'e', help = "Expand archives and procedurals")]
    pub expand: bool,

    #[clap(long = "expand-archives", help = "Expand archives")]
    pub expand_archives: bool,

    #[clap(long = "expand-procedurals", help = "Expand procedurals")]
    pub expand_procedurals: bool,

    #[clap(name = "FILE", help = "The NSI FILE(s) to dump")]
    pub file: Option<String>,

    #[clap(
        long,
        short = 'o',
        help = "Dump NSI stream to OUTPUT",
        long_help = "Dump NSI stream to OUTPUT instead of stdout"
    )]
    pub output: Option<String>,
}
