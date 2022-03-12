# 3`delight-helpers`

[3Delight](https://www.3delight.com/) command line utilities/helpers.

For now just a `renderdl` replacement.

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install).

2. Run:
   ```
   cargo install delight-helpers
   ```

## Helpers

### `rdl`

```
rdl
Renders or filters NSI streams or Lua NSI files

USAGE:
    rdl <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    cat                     Dump the input as an NSI stream to stdout/a file
    help                    Print this message or the help of the given subcommand(s)
    render                  Display the renderer version
    generate-completions    Generate completion scripts for various shells

‘rdl -h’ prints a brief overview while ‘rdl --help’ gives all details
```

#### `render` Subcommand

```
rdl-render
Render an image of result with 3Delight

USAGE:
    rdl render [OPTIONS] [FILE]...

ARGS:
    <FILE>...
            The NSI FILE(s) to render
            Frame number placeholders are specified using @[padding]:
            foo.@.nsi  ⟶   foo.1.nsi, foo.2.nsi, …
            foo.@4.nsi ⟶   foo.0001.nsi, foo.0002.nsi, …

            Globbing using "<pattern>" (in quotes) is supported -
            even if your shell has no support for it:
            "**/{*.{nsi,lua}}" ⟶ all .nsi and .lua files in the
            current folder and its subfolders

OPTIONS:
    -c, --cloud
            Use 3Delight Cloud to render the specified file(s)

    -d, --display
            Send 3Delight Display (a copy of) the image(s) being
            rendered.

        --dry-run
            Do not render, just print the name of the file(s) to be
            rendered.

    -f, --frames <FRAMES>
            FRAME(S) to render
            They can be specified individually:
            1,2,3,5,8,13
            Or as a squence:
            10-15   ⟶   10, 11, 12, 13, 14, 15
            With an optional step size:
            10-20@2 ⟶   10, 12, 14, 16, 18, 20
            Step size is always positive.
            To render a sequence backwards specify the range in reverse:
            42-33@3 ⟶   42, 39, 36, 33
            With binary splitting. Useful to quickly check if a sequence
            has ‘issues’ in some frames:
            10-20@b ⟶   10, 20, 15, 12, 17, 11, 13, 16, 18, 14, 19
            The last frame of a sequence will be omitted if
            the specified step size does not touch it:
            80-70@4 ⟶   80, 76, 72

    -h, --help
            Print help information

    -p, --progress
            Print rendering progress at each bucket

    -t, --threads <THREADS>
            Launch the render using number of THREADS

    -u, --using <USING>
            Render using the local machine (default):
            --using local
            Render using the cloud:
            --using cloud
            Render using the colletive `Molodchy`:
            --using Molodchy

    -v, --verbose
            Print the names of the file(s) being rendered
```

#### `cat` Subcommand

```
rdl-cat
Dump the input as an NSI stream to stdout/a file

USAGE:
    rdl cat [OPTIONS] [FILE]

ARGS:
    <FILE>
            The NSI FILE(s) to dump

OPTIONS:
    -b, --binary
            Encode NSI stream in binary format

    -e, --expand
            Expand archives and procedurals

        --expand-archives
            Expand archives

        --expand-procedurals
            Expand procedurals

    -g, --gzip
            Compress NSI stream using GNU zip format

    -h, --help
            Print help information

    -o, --output <OUTPUT>
            Dump NSI stream to OUTPUT instead of stdout
```

#### `generate-completions` Subcommand

```
rdl-generate-completions
Generate completion scripts for various shells

USAGE:
    rdl generate-completions <SHELL>

ARGS:
    <SHELL>    The shell to generate completions for [possible values: bash, fish, zsh, powershell, elvish]

OPTIONS:
    -h, --help    Print help information
```
