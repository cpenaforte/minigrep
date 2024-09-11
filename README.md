# Mini grep

## What it do
Mini grep is a command-line tool to get lines of a text file that contains a substring.

## Usage
Run `cargo run -- query file_path`

Where:
- **query**: The substring to be searched.
- **file_path**: The path to the text file.

_Note that there is an example file named poem.txt_

### Case-insensitive search
Run `IGNORED_CASE=1 cargo run -- query file_path`

### Output to another file
Run `cargo run -- query file_path > output_path`

Where:
- **output_path**: The path to the output text file.