# Wispha

![Build Status](https://travis-ci.org/Evian-Zhang/Wispha.svg?branch=master)![language](https://img.shields.io/badge/language-rust-orange.svg)

Wispha is a commandline tool for easily displaying project layout.

Other versions:

* [简体中文](README-zh-CN.md)

## Usage

If you have wispha in path, clone this project and type the following command inside this directory:

```shell
wispha layout --file LOOKME.json --project-name Wispha
```

And you will see this project layout:

```
Wispha
├── libwispha
│   ├── src
│   │   ├── lib.rs
│   │   ├── core.rs
│   │   ├── strings.rs
│   │   ├── manipulator.rs
│   │   └── serde
│   ├── tests
│   │   ├── ser_test
│   │   └── de_test
│   └── Cargo.toml
├── src
│   ├── main.rs
│   ├── layouter.rs
│   ├── layout_templates
│   │   ├── mod.rs
│   │   ├── plain.rs
│   │   ├── line.rs
│   │   └── triangle.rs
│   └── commandline
│       ├── mod.rs
│       ├── layout.rs
│       └── interact
├── Cargo.toml
├── README.md
├── README-zh-CN.md
├── LICENSE-MIT
└── LICENSE-APACHE
```

And you can use interact mode to get more information. Just type

```shell
wispha interact --file LOOKME.json --project-name Wispha
```

Then you will get into interact mode. Use the following command to get more infomation of `src/main.rs`:

```shell
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
      "name": "Cargo.toml"
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
    }
  ]
}
```

For a simple file, such as `LICENSE-MIT`, you can just add an object to `children` key's list, and `description` key is optional and you can name any key (for key's rule, see [wiki](https://github.com/Evian-Zhang/Wispha/wiki)) as you wish.

For a folder, if this folder contains many files and you don't want this to make your original `LOOKME.json` ugly, you can add `type` key with string value `Link`, and set its `target` to be the path of another file with the similar structure. Or you can simply add `children` to the object as [src/LOOKME.json](src/LOOKME.json) does.

## Contributing to Wispha

Welcome to make issues and pull request.