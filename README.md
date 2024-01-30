# sqlstitch

`sqlstitch` is a command-line tool that reads SQL files, sorts them based on their foreign key constraints, and prints the sorted SQL statements to the standard output. The output is safe to execute as is, which means it can be directly used to create or modify a database schema.

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

## Usage

You can use `sqlstitch` by providing it with one or more SQL files as arguments. Here's the basic usage:

```shell
$ sqlstich <FILES>...
```

For example, if you have a directory named schema containing your SQL files, you can sort all of them and write the output to a new SQL file like this:

```shell
$ sqlstich schema/*.sql > sorted_schema.sql
```

This will create a new file named sorted_schema.sql containing the sorted SQL statements.

## Installation

To install `sqlstitch`, please refer to the [releases](https://github.com/yshrsmz/sqlstitch/releases) page. You'll find the latest version of sqlstitch there, along with instructions on how to install it.

## Contributing

If you'd like to contribute to the development of sqlstitch, we'd love to have your help! You can start by checking out our open issues. If you find one that you'd like to work on, feel free to fork the repository and submit a pull request.


## License

`sqlstitch` is licensed under the [Apache-2.0 License](https://opensource.org/license/apache-2-0/).
