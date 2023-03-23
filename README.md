# SearchR CLI

A command-line interface (CLI) tool for creating the command to rename all the references of a URL in a WordPress database.

## Installation

To use the SearchR CLI, you will need to have Rust installed on your system. If you don't have Rust installed, you can install it from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Once you have Rust installed, you can install the SearchR CLI by running the following command:

bashCopy code

`$ cargo install --git https://github.com/sodesautels/searchr-cli.git`

## Usage

To use the SearchR CLI, simply run the `searchr` command followed by the old URL and the new URL:

bashCopy code

`$ searchr <old_url> <new_url>`

For example:

bashCopy code

`$ searchr https://oldurl.com https://newurl.com`

This will output the command that you can use to rename all the references of the old URL to the new URL in a WordPress database. The command will be copied to your clipboard automatically.

The generated command includes the following options:

- `--all-tables`: Search and replace in all tables in the database.
- `--verbose`: Show the results of the search and replace operation.
- `--precise`: Do a case-sensitive, whole-word search.
- `--dry-run`: Perform a dry run of the search and replace operation, showing what would be replaced without actually making any changes to the database.
