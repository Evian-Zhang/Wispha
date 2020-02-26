# Wispha

![Build Status](https://travis-ci.org/Evian-Zhang/Wispha.svg?branch=master)![language](https://img.shields.io/badge/language-rust-orange.svg)

Wispha是一个可以轻松展示项目结构布局的命令行工具。

## 安装

### Windows

### macOS

您可以使用homebrew进行下载：

```shell script
brew tap Evian-Zhang/Wispha
brew install wispha
```

### Linux

## 用法

如果wispha在你的路径中，那么请将本项目克隆至自己的电脑中，然后在该目录下输入命令

```shell
wispha layout --file LOOKME.json --project-name Wispha
```

然后就可以看到本项目的结构布局：

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

对于一个简单的文件，比如`LICENSE-MIT`, 可以直接在`children` key对应的列表中添加一个对象。其中，`description` key不是必要的，同时你可以任意添加键（键的要求请参考[wiki](https://github.com/Evian-Zhang/Wispha/wiki)）。

对于一个文件夹，如果该文件夹中包含了过多的文件，同时你并不想因此使`LOOKME.json`文件变得臃肿，你可以添加`type` key, 并将其值设置为字符串`Link`, 然后添加`target`键，其值为对应的JSON文件所在的路径。除此之外，你还可以直接在该对象下添加`children`，如[src/LOOKME.json](src/LOOKME.json).

## 贡献

欢迎提issue和pull request.