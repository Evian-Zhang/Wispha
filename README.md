# Wispha

![Build Status](https://travis-ci.org/Evian-Zhang/Wispha.svg?branch=master)![language](https://img.shields.io/badge/language-rust-orange.svg)

Wispha is a commandline tool for easily displaying project layout.

Other versions:

* [简体中文](README-zh-CN.md)



## Why Wispha

With more and more projects becoming open source, we may meet many big, complicated projects, which may have hundreds of thoudsands of files and directories. However, many projects don't have detailed descriptions of those files and directories, which makes us difficult to read the source code. And for those who have, the display of those descriptions is not standardized. So I want to use this tool to help.

## Installation

### Windows

Install from [latest release](https://github.com/Evian-Zhang/Wispha/releases/latest/download/wispha-win10.tar.gz).

### macOS

You can use homebrew to install wispha:

```shell script
brew tap Evian-Zhang/Wispha
brew install wispha
```

### Debian/Ubuntu

Install the deb file from [latest release](https://github.com/Evian-Zhang/Wispha/releases/latest/download/wispha.deb).

### Build from source

Make sure your rust is latest stable. Clone this repository, and inside the directory:

```shell script
cargo build --release
```

## Usage

If you have wispha in path, clone this project and type the following command inside this directory:

```shell script
wispha layout --project-name Wispha --keys description --hide-key
```

And you will see this project layout:

```
Wispha                        Wispha project main folder
├── libwispha                 Wispha library used by binary wispha
│   ├── src                   Source code of library wispha.
│   │   ├── lib.rs            Entry point for the library
│   │   ├── core.rs           Define core structs
│   │   ├── strings.rs        Consists of static str used by library
│   │   ├── manipulator.rs    APIs for node manipulation
│   │   └── serde             APIs for serialization and deserialization
│   ├── tests                 integration tests
│   │   ├── ser_test          tests for serialization
│   │   └── de_test           tests for deserialization
│   ├── Cargo.toml            Manifest file for cargo to run
│   ├── LICENSE-MIT           MIT license
│   └── LICENSE-APACHE        Apache license version 2.0
├── src                       Source code of binary executable wispha.
│   ├── main.rs               Entry point of binary
│   ├── layouter.rs           Define the `Layout` trait for templates
│   ├── layout_templates      Templates that implements `Layout` trait
│   │   ├── mod.rs            
│   │   ├── plain.rs          Define the plain layout.
│   │   ├── line.rs           Define the line layout.
│   │   └── triangle.rs       Define the triangle layout.
│   └── commandline           Commandline interface
│       ├── mod.rs            
│       ├── layout.rs         Layout subcommand
│       └── interact          Interact subcommand
├── Cargo.toml                Manifest file for cargo to run
├── README.md                 
├── README-zh-CN.md           Simplified Chinese version of README
├── LICENSE-MIT               MIT license
├── LICENSE-APACHE            Apache license version 2.0
└── .travis.yml               File for Travis CI to run
```

And you can use interact mode to get more information. Just type

```shell script
wispha interact --file LOOKME.json --project-name Wispha
```

Then you will get into interact mode. Use the following command to get more infomation of `src/main.rs`:

```shell script
(wispha) get --key description --path /src/main.rs
```

You will get

```
Entry point of binary
```

and enter `quit` to quit interact mode.

For more documentation, please see [wiki](https://github.com/Evian-Zhang/Wispha/wiki).

## Apply Wispha to your own project

To apply Wispha to your own project, just add JSON files containing project information.

For example, let's look at this project's `LOOKME.json` (you can name this JSON file anything you want, and I prefer `LOOKME.json`):

```json
{
  "description": "Wispha project main folder",
  "children": [
    {
      "name": "libwispha",
      "type": "Link",
      "target": "libwispha/LOOKME.json"
    },
    {
      "name": "src",
      "type": "Link",
      "target": "src/LOOKME.json"
    },
    {
      "name": "Cargo.toml",
      "description": "Manifest file for cargo to run"
    },
    {
      "name": "README.md"
    },
    {
      "name": "README-zh-CN.md",
      "description": "Simplified Chinese version of README"
    },
    {
      "name": "LICENSE-MIT",
      "description": "MIT license"
    },
    {
      "name": "LICENSE-APACHE",
      "description": "Apache license version 2.0"
    },
    {
      "name": ".travis.yml",
      "description": "File for Travis CI to run"
    }
  ]
}
```

For a simple file, such as `LICENSE-MIT`, you can just add an object to `children` key's list, and `description` key is optional and you can name any key (for key's rule, see [wiki](https://github.com/Evian-Zhang/Wispha/wiki)) as you wish.

For a folder, if this folder contains many files and you don't want this to make your original `LOOKME.json` ugly, you can add `type` key with string value `Link`, and set its `target` to be the path of another file with the similar structure. Or you can simply add `children` to the object as [src/LOOKME.json](src/LOOKME.json) does.

## Contributing to Wispha

Welcome to open issues or make pull request.