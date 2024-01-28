# sqlstitch

sqlstitch reads provided SQL files to sort by their foreign key constraints and print to standard output. Output should be safe to execute as is.

```shell
$ sqlstitch --help
Usage: sqlstich [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Input files to process

Options:
  -v, --verbose  Prints debug information
  -h, --help     Print help
  -V, --version  Print version
```
