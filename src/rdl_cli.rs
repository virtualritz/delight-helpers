use clap::{Parser, Subcommand};

#[inline]
pub fn build_cli() -> Cli {
    Cli::parse()
}

#[derive(Parser)]
#[command(
    name = "rdl",
    bin_name = "rdl",
    about = "Renders or filters NSI streams or Lua NSI files with 3Delight",
    after_long_help = "",
    max_term_width = 120
)]
pub struct Cli {
    #[arg(
        display_order = 10,
        long,
        short,
        action = clap::ArgAction::Count,
        help = "Verbosity level (-v verbose, -vv very verbose, etc.)",
    )]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Render(Render),
    Cat(Cat),
    Watch(Watch),
    #[command(
        name = "version",
        about = "Display version information",
        display_order = 9998
    )]
    Version,
    //Usd(Usd),
    #[command(
        name = "generate-completions",
        about = "Generate completion scripts for various shells",
        display_order = 9999
    )]
    GenerateCompletions {
        #[arg(
            help = "The shell to generate completions for",
            value_parser = clap::builder::PossibleValuesParser::new([
                "bash",
                "elvish",
                #[cfg(feature = "fig")]
                "fig",
                "fish",
                #[cfg(feature = "nushell")]
                "nushell",
                "powershell",
                "zsh"])
        )]
        shell: String,
    },
}

#[derive(Parser, Clone)]
#[command(
    arg_required_else_help = true,
    about = "Render NSI file(s) with 3Delight",
//    help_message = "Print this/a long help message."
)]
pub struct Render {
    #[arg(
        name = "FILE",
        index = 1,
        help = "The NSI FILE(s) to render",
        long_help = "The NSI FILE(s) to render\n\
            Frame number placeholders are specified using @[padding]:\n\
            foo.@.nsi   ➞  foo.1.nsi, foo.2.nsi, …\n\
            foo.@4.nsi  ➞  foo.0001.nsi, foo.0002.nsi, …",
        value_hint = clap::ValueHint::FilePath
    )]
    pub file: Vec<String>,

    #[arg(
        long,
        short = 'C',
        conflicts_with = "cloud",
        help = "Use the given COLLECTIVE",
        long_help = "Render using the given 3Delight COLLECTIVE"
    )]
    pub collective: Option<String>,

    #[arg(
        long,
        short,
        conflicts_with = "collective",
        help = "Use 3Delight Cloud",
        long_help = "Render using 3Delight Cloud"
    )]
    pub cloud: bool,

    /*
    #[arg(
        long,
        short,
        help = "Send the image(s) being rendered to 3Delight Display",
        long_help = "Send 3Delight Display (a copy of) the image(s) being \
            rendered."
    )]
    pub display: bool,
    */
    #[arg(long, short, help = "Print rendering progress at each bucket")]
    pub progress: bool,

    #[arg(
        long,
        short,
        action = clap::ArgAction::Count,
        help = "Generate statistics: (-s, -ss)",
        long_help = "Statistics level\n\
        -s   ➞  embed in image\n\
        -ss  ➞  embed in image & print to stdout",
    )]
    pub statistics: u8,

    #[arg(
        long,
        short,
        help = "Launch the render using number of THREADS",
        long_help = "Launch the render using number of THREADS\n\
            If not specified the number of threads will be determined by the \
            COLLECTIVE or the number of cores on the machine."
    )]
    pub threads: Option<usize>,

    /*
    #[arg(
        long,
        short,
        help = "Display a rendering progress bar")]
    pub progress: bool,
    */
    #[arg(
        long,
        help = "Do not render, just print what would be done",
        long_help = "Do not render, just print the name of the file(s) to be \
            rendered"
    )]
    pub dry_run: bool,

    #[arg(
        long,
        help = "Add a render command to the NSI stream",
        long_help = "Add a render command to the NSI stream\n\
            Useful when the stream is missing this command.\n\
            This doesn't check if the stream already has a render command. If \
            it does this may cause parts or all of the stream to render \
            twice."
    )]
    pub force_render: bool,

    #[arg(
        long,
        short,
        help = "FRAME(s) to render – 1,2,10-20,40-30@2",
        long_help = "FRAME(S) to render\n\
            They can be specified individually:\n\
            1,2,3,5,8,13\n\
            Or as a squence:\n\
            10-15    ➞  10, 11, 12, 13, 14, 15\n\
            With an optional step size:\n\
            10-20@2  ➞  10, 12, 14, 16, 18, 20\n\
            Step size is always positive.\n\
            To render a sequence backwards specify the range in reverse:\n\
            42-33@3  ➞  42, 39, 36, 33\n\
            With binary splitting. Useful to quickly check if a sequence\n\
            has ‘issues’ in some frames:\n\
            10-20@b  ➞  10, 20, 15, 12, 17, 11, 13, 16, 18, 14, 19\n\
            The last frame of a sequence will be omitted if\n\
            the specified step size does not touch it:\n\
            80-70@4  ➞  80, 76, 72"
    )]
    pub frames: Option<String>,
}

#[derive(Parser)]
#[command(
    arg_required_else_help = true,
    about = "Dump the input as an NSI stream to stdout or a file",
//    help_message = "Print this help message."
)]
pub struct Cat {
    #[arg(long, short, help = "Encode NSI stream in binary format")]
    pub binary: bool,

    #[arg(long, short, help = "Compress NSI stream using GNU zip format")]
    pub gzip: bool,

    #[arg(long, short, help = "Expand archives and procedurals")]
    pub expand: bool,

    //#[arg(long = "expand-archives", short = 'a', help = "Expand archives")]
    //pub expand_archives: bool,

    //#[arg(long = "expand-procedurals", short = 'p', help = "Expand procedurals")]
    //pub expand_procedurals: bool,
    #[arg(
        name = "FILE",
        help = "The NSI FILE to dump",
        value_hint = clap::ValueHint::FilePath
    )]
    pub file: Option<String>,

    #[arg(
        long,
        short,
        help = "Dump NSI stream to OUTPUT",
        long_help = "Dump NSI stream to OUTPUT instead of stdout"
    )]
    pub output: Option<String>,
}

#[derive(Parser)]
#[command(
    arg_required_else_help = true,
    about = "Watch folder(s) for new files and render them with 3Delight",
//    help_message = "Print this/a long help message."
)]
pub struct Watch {
    #[arg(
        name = "FOLDER",
        index = 1,
        help = "The FOLDER(s) to watch for NSI files(s) to render",
        value_hint = clap::ValueHint::DirPath
    )]
    pub folder: Vec<String>,

    #[arg(
        long,
        short = 'C',
        conflicts_with = "cloud",
        help = "Render using the the given 3Delight COLLECTIVE"
    )]
    pub collective: Option<String>,

    #[arg(
        long,
        short,
        conflicts_with = "collective",
        help = "Render using 3Delight Cloud"
    )]
    pub cloud: bool,

    #[arg(
        long,
        short,
        help = "Recurse into the given folder(s)",
        long_help = "Recurse into the given folder(s) when looking for new files to render"
    )]
    pub recursive: bool,
}
