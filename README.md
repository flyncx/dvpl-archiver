# DVPL Archiver
A CLI tool to unpack/pack DVPL Resource Archive.

### Installation
1. Clone this repository;
2. CD into it;
3. Make sure the source code is gud;
3. Run `cargo install --path .`


### CLI
`dvpl-archiver help`
```sh
Usage: dvpl-archiver <COMMAND>

Commands:
  pack    Pack input file into DVPL Resource Archive
  unpack  Unpack DVPL Resource Archive
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

`dvpl-archiver pack`
```bash
Pack input file into DVPL Resource Archive

Usage: dvpl-archiver pack <file>

Arguments:
  <file>  Input file path

Options:
  -h, --help  Print help
```

`dvpl-archiver unpack`
```bash
Unpack DVPL Resource Archive

Usage: dvpl-archiver unpack <file>

Arguments:
  <file>  DVPL Resource Archive path

Options:
  -h, --help  Print help
```

### Reference
- [ResourceArchiver.h](https://github.com/rifsxd/dava.engine.framework/blob/master/Modules/ResourceArchiverModule/Sources/ResourceArchiverModule/ResourceArchiver.h)
- [ResourceArchiver.cpp](https://github.com/rifsxd/dava.engine.framework/blob/master/Modules/ResourceArchiverModule/Sources/ResourceArchiverModule/Private/ResourceArchiver.cpp)

### License
MIT. [See LICENSE file](./LICENSE)
