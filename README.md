# Why does this crate exists?

At work, I was looking for a tool to convert a java property file into a yaml formatted property file.

- since I didn't find any good cli tool for the job I wrote my own tool
- I was also interested in this new cool hip programming language called **rust**.

That's being said I am a noob when it comes to rust development. So please be nice 😁

# Current state
![Rust project pipeline](https://github.com/Ben7X/procon/actions/workflows/rust.yml/badge.svg)

## General

- I already have unit and integration tests but bugs can still occur, so all in testing phase
- If you find any bugs please let me know on my GitHub project
    - please also leave the file content you wanted to convert in the bug report, so I can reproduce the bug

## Supported conversions

- Conversion from **property file** to **yaml** ✔️
- Conversion from **property file** to **json** ✔️
- Conversion from **yaml** to **property file** ✔️
- Conversion from **yaml** to **json**  ✔️
- Conversion from **json** to **yaml** ✔️
- Conversion from **json** to **property file** ✔️

# How to use

- convert test.properties file to json

```shell
procon json test.properties
```

- convert stdout as input for procon
- the -j flag defines the property format of stdin bytes as json, -y yaml, -p property

```shell
cat test.json | procon -j yaml -
```

# What's coming next

- Bug fixes if there are any
- internal refactorings
    - in regard to https://rust-cli.github.io/book/index.html
    - refactor root list conversion code and potential bugs
- add toml property format support
- interactive mode with editor functionality

# Releases

## 0.2.6

- add github pipeline badge
 
## 0.2.5

- change log4rs for env_logger
- accept stdin as argument
- refactoring code base in regard to https://rust-cli.github.io/book/index.html

## 0.2.4

- ability to parse yaml now
- restructured code base to rust conventions
- added integration test

# You want to help

- If you want to help in the development feel free
- If you are a pro rust developer, and you have some tips appreciate it