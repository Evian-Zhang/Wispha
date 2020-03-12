# Wispha

![Build Status](https://travis-ci.org/Evian-Zhang/Wispha.svg?branch=master)![language](https://img.shields.io/badge/language-rust-orange.svg)

Wispha是一个可以轻松展示项目结构布局的命令行工具。

## 为什么使用Wispha

随着越来越多的项目开源，我们可以接触到许多大型的、复杂的项目。这些项目可能会有成百上千个文件、目录。但是，许多项目对于这些文件和目录并没有详细的说明，让我们阅读源码很困难。而那些有详细说明的项目，其展示这些说明的方法也并不统一。因此，我希望用这个工具来改善这一现状。

## 安装

### Windows

您可以直接下载[最新的release](https://github.com/Evian-Zhang/Wispha/releases/latest/download/wispha-installer.msi).

### macOS

您可以使用homebrew进行下载：

```shell script
brew tap Evian-Zhang/Wispha
brew install wispha
```

### Debian/Ubuntu

您可以直接下载[最新的release](https://github.com/Evian-Zhang/Wispha/releases/latest/download/wispha.deb).

### 从源码编译

请确保您电脑中的Rust是最新的stable版本。将本仓库克隆至本地，然后在其目录下使用

```shell script
cargo build --release
```

## 用法

如果wispha在你的路径中，那么请将本项目克隆至自己的电脑中，然后在该目录下输入命令

```shell script
wispha layout --project-name Wispha --keys description --hide-key
```

然后就可以看到本项目的结构布局：

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

同时，你也可以使用交互模式获得更多的信息。只需输入

```shell
wispha interact --file LOOKME.json --project-name Wispha
```

然后就可以进入交互模式了。使用如下命令获得`src/main.rs`的更多信息：

```shell
(wispha) get --key description --path /src/main.rs
```

得到

```
Entry point of binary
```

输入`quit`退出交互模式。

请参看[wiki](https://github.com/Evian-Zhang/Wispha/wiki)查看更多指南。

## 在你自己的项目中使用Wispha

如需在自己项目中使用Wispha, 只需添加包含项目布局信息的JSON文件即可。

例如本项目的`LOOKME.json`文件（你可以使用任何名字命名这个JSON文件，只是我更喜欢`LOOKME.json`这个名字）：

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

对于一个简单的文件，比如`LICENSE-MIT`, 可以直接在`children` key对应的列表中添加一个对象。其中，`description` key不是必要的，同时你可以任意添加键（键的要求请参考[wiki](https://github.com/Evian-Zhang/Wispha/wiki)）。

对于一个文件夹，如果该文件夹中包含了过多的文件，同时你并不想因此使`LOOKME.json`文件变得臃肿，你可以添加`type` key, 并将其值设置为字符串`Link`, 然后添加`target`键，其值为对应的JSON文件所在的路径。除此之外，你还可以直接在该对象下添加`children`，如[src/LOOKME.json](src/LOOKME.json).

## 贡献

欢迎提issue和pull request.