# Wispha

![language](https://img.shields.io/badge/language-rust-orange.svg)

Wispha is a commanline tool for easily displaying project layout.

## Usage

If you have wispha in path, clone this project and type the following command inside this directory:

```bash
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
└── LICENSE
```

Or you can use interact mode to get more information. Just type

```bash
wispha interact --file LOOKME.json --project-name Wispha
```

Then you will get into interact mode. Use the following command to get more infomation of `src/main.rs`:

```bash
(wispha) get --key description --path /src/main.rs
```

You will get

```
Entry point of binary
```