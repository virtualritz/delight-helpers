# 3`delight-helpers`

[3Delight](https://www.3delight.com/) command line utilities/helpers.

For now just a `renderdl` replacement.

* [Installation](#installation)
* [Helpers](#helpers)
  * [`rdl`](#rdl)
    * [`render` Subcommand](#render-subcommand)
    * [`cat` Subcommand](#cat-subcommand)
    * [`watch` Subcommand](#watch-subcommand)
    * [`generate-completions` Subcommand](#generate-completions-subcommand)

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install).

2. Run:
   ```
   cargo install delight-helpers
   ```

## Helpers

### `rdl`

```
Renders or filters NSI streams or Lua NSI files with 3Delight

Usage: rdl [OPTIONS] <COMMAND>

Commands:
  render                Render NSI file(s) with 3Delight
  cat                   Dump the input as an NSI stream to stdout or a file
  watch                 Watch folder(s) for new files and render them with 3Delight
  help                  Print this message or the help of the given subcommand(s)
  version               Display version information
  generate-completions  Generate completion scripts for various shells

Options:
  -v, --verbose...
          Verbosity level (-v verbose, -vv very verbose, etc.)

  -h, --help
          Print help (see a summary with '-h')
```

#### `render` Subcommand

```
Render NSI file(s) with 3Delight

Usage: rdl render [OPTIONS] [FILE]...

Arguments:
  [FILE]...
          The NSI FILE(s) to render
          Frame number placeholders are specified using @[padding]:
          foo.@.nsi   ➞  foo.1.nsi, foo.2.nsi, …
          foo.@4.nsi  ➞  foo.0001.nsi, foo.0002.nsi, …

Options:
  -C, --collective <COLLECTIVE>
          Render using the given 3Delight COLLECTIVE

  -c, --cloud
          Render using 3Delight Cloud

  -p, --progress
          Print rendering progress at each bucket

  -s, --statistics...
          Statistics level
          -s   ➞  embed in image
          -ss  ➞  embed in image & print to stdout

  -t, --threads <THREADS>
          Launch the render using number of THREADS
          If not specified the number of threads will be determined by the COLLECTIVE or the number of cores on the
          machine.

      --dry-run
          Do not render, just print the name of the file(s) to be rendered

      --force-render
          Add a render command to the NSI stream
          Useful when the stream is missing this command.
          This doesn't check if the stream already has a render command. If it does this may cause parts or all of the
          stream to render twice.

  -f, --frames <FRAMES>
          FRAME(S) to render
          They can be specified individually:
          1,2,3,5,8,13
          Or as a squence:
          10-15    ➞  10, 11, 12, 13, 14, 15
          With an optional step size:
          10-20@2  ➞  10, 12, 14, 16, 18, 20
          Step size is always positive.
          To render a sequence backwards specify the range in reverse:
          42-33@3  ➞  42, 39, 36, 33
          With binary splitting. Useful to quickly check if a sequence
          has ‘issues’ in some frames:
          10-20@b  ➞  10, 20, 15, 12, 17, 11, 13, 16, 18, 14, 19
          The last frame of a sequence will be omitted if
          the specified step size does not touch it:
          80-70@4  ➞  80, 76, 72

  -h, --help
          Print help (see a summary with '-h')
```

#### `cat` Subcommand

```
Dump the input as an NSI stream to stdout or a file

Usage: rdl cat [OPTIONS] [FILE]

Arguments:
  [FILE]
          The NSI FILE to dump

Options:
  -b, --binary
          Encode NSI stream in binary format

  -g, --gzip
          Compress NSI stream using GNU zip format

  -e, --expand
          Expand archives and procedurals

  -o, --output <OUTPUT>
          Dump NSI stream to OUTPUT instead of stdout

  -h, --help
          Print help (see a summary with '-h')
```

#### `watch` Subcommand

```
Watch folder(s) for new files and render them with 3Delight

Usage: rdl watch [OPTIONS] [FOLDER]...

Arguments:
  [FOLDER]...
          The FOLDER(s) to watch for NSI files(s) to render

Options:
  -C, --collective <COLLECTIVE>
          Render using the the given 3Delight COLLECTIVE

  -c, --cloud
          Render using 3Delight Cloud

  -r, --recursive
          Recurse into the given folder(s) when looking for new files to render

  -h, --help
          Print help (see a summary with '-h')
```

#### `generate-completions` Subcommand

```
Generate completion scripts for various shells

Usage: rdl generate-completions <SHELL>

Arguments:
  <SHELL>  The shell to generate completions for [possible values: bash, elvish, fig, fish, nushell, powershell, zsh]

Options:
  -h, --help  Print help
```

For example, if you use [`oh-my-zsh`](https://ohmyz.sh/), you can install completions by running:

```
rdl generate-completions zsh >~/.oh-my-zsh/completions/_rdl
```
