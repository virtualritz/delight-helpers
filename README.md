# `rdl`

3Delight command line client/renderdl replacement.

# Invocation

```
rdl 0.1.0
Renders or filters NSI streams or Lua NSI files

USAGE:
    rdl [CONFIG]
    rdl <SUBCOMMAND>

OPTIONS:
    -h, --help       Print this help message.
    -V, --version    Print version information.

ARGS:
    <CONFIG>

SUBCOMMANDS:
    render    Render an image of result with 3Delight.
    cat       Dump the input to stdout as an NSI stream.

Note: ‘rdl -h’ prints a short and concise overview while ‘rdl --help’
gives all details.
```
## `render` Subcommand

```
rdl-render
Render an image of result with 3Delight.

USAGE:
    rdl render [OPTIONS] <FILE>...
    rdl render <SUBCOMMAND>

OPTIONS:
        --cloud
            Use 3Delight Cloud to render the specified files.

    -d, --display
            Send 3Delight Display (a copy of) all rendered images.

    -v, --verbose
            Print the names of the files being rendered.

    -p, --progress
            Print rendering progress at each bucket.

    -q, --quiet
            Do not print the name of the file being rendered.

        --embed-statistics
            Embed statistics in rendered images.

    -f, --frame <FRAME>
            FRAME(s) to render.
            They can be specified individually:
            1,2,3,5,8,13
            Or as a squence:
            10-15   ⟶   10, 11, 12, 13, 14, 15
            With an optional step size:
            10-20@2 ⟶   10, 12, 14, 16, 18, 20
            Step size is always positive.
            To render a sequence backwards specify the range
            in reverse:
            42-37   ⟶   42, 41, 40, 39, 38, 37
            The last frame of a sequence will be omitted if
            the specified step size does not touch it:
            80-70@4 ⟶   80, 76, 72

    -h, --help
            Print this/a long help message.


ARGS:
    <FILE>...
            The NSI FILE(s) to render.
            Frame number placeholders are specified using @[padding]:
            foo.@4.nsi ⟶   foo.0001.nsi, foo.0002.nsi, …
```

## `cat` Subcommand

```
rdl-cat
Dump the input to stdout as an NSI stream.

USAGE:
    rdl cat [OPTIONS] <FILE>
    rdl cat <SUBCOMMAND>

OPTIONS:
    -b, --binary                Encode NSI stream in binary format.
    -g, --gzip                  Compress NSI stream using Gzip format.
    -e, --expand                Expand archives and procedurals.
        --expand-archives       Expand archives only.
        --expand-procedurals    Expand procedurals only.
    -o, --output <OUTPUT>       Dump NSI stream to OUTPUT instead of stdout.
    -h, --help                  Print this help message.

ARGS:
    <FILE>    The NSI FILE(s) to dump.
```