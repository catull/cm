[![Build Status](https://github.com/tsoding/cm/workflows/CI/badge.svg)](https://github.com/tsoding/cm/actions)

# cm

**WARNING! The application is in an active development state and is not even alpha yet. Use it at your own risk. Nothing is documented, anything can be changed at any moment or stop working at all.**

[![asciicast](https://asciinema.org/a/327091.svg)](https://asciinema.org/a/327091)

The goal of this application is to recreate the [compilation-mode] of Emacs as a standalone TUI application.

## Build

### Dependencies

- [rust](https://www.rust-lang.org/)
- [ncurses](https://invisible-island.net/ncurses/)
- [pcre2](https://www.pcre.org/)

### Workflow

```console
$ cargo build
$ ./target/debug/cm -c "grep -rn 'String' src/"
```

## Usage

### Starting up

<!-- TODO(#85): Document start up process -->
TBD

### Application layout

<!-- TODO: Document application layout -->
TBD

### Shortcuts

<!-- TODO: Document shortcuts -->
TBD

### Configuration file cm.conf

<!-- TODO(#45): Document config format -->
[TBD](https://github.com/tsoding/cm/issues/45)

## Examples

**Fixing compilation errors**
[![asciicast](https://asciinema.org/a/337846.svg)](https://asciinema.org/a/337846)

[compilation-mode]: https://www.gnu.org/software/emacs/manual/html_node/emacs/Compilation-Mode.html
