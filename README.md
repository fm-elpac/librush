# 艾刷 (libRush)
<https://github.com/fm-elpac/librush>

镜像 (mirror):
+ <https://bitbucket.org/fm-elpac/librush/>
+ <https://codeberg.org/fm-elpac/librush>
+ <https://notabug.org/fm-elpac/librush>
+ <https://gitlab.com/fm-elpac/librush>

----

艾刷 (libRush = lib + IBus + Rust + h):
用 rust 编写的 ibus 模块, 不用 GObject
(ibus module written in pure rust, without GObject)

![CI](https://github.com/fm-elpac/librush/actions/workflows/ci.yml/badge.svg)

----

ibus 输入法框架使用 C 语言和 GObject 编写,
并不方便在 rust 编程语言之中使用.
于是又造了这个轮子. (狗头)

The ibus input method framework is written in C and GObject.
This project is created for easy use of ibus in rust.


## 文档 (Document)

+ <https://docs.rs/librush/>

+ 《ibus 源代码阅读 (1)》
  - <https://blog.csdn.net/secext2022/article/details/136099328>
  - <https://zhuanlan.zhihu.com/p/682125811>
  - <https://www.bilibili.com/read/cv31187008/>


## 国际化 (i18n)

+ 项目第一语言: 简体中文 (`zh_CN`)

  project first language: Chinese

+ 项目第二语言: 英文 (`en`)

  project second language: English

+ 外部编程接口语言: 英文

  public API language: English

+ 项目主要开发者位置: 中国

  main author location: China

  项目类型: 个人

  project type: personal


## 贡献 (Contribute)

对此项目的贡献 (提问题, 提交代码等) 尽量在主平台 (github) 进行.
在镜像平台的贡献很可能会被忽略.

Please contribute (issue, PR, etc.) at the main platform (github).
The mirrors can be ignored.


## 相关链接 (Links)

+ IBus: Intelligent Input Bus for Linux/Unix

  <https://github.com/ibus/ibus>

+ D-Bus is a message bus system, a simple way for applications to talk to one another

  <https://www.freedesktop.org/wiki/Software/dbus/>

+ zbus: A Rust API for D-Bus communication

  <https://github.com/dbus2/zbus/>

+ Rust Programming Language

  <https://www.rust-lang.org/>

+ GObject: The GLib Object System

  <https://docs.gtk.org/gobject/>

+ GNOME Shell: next generation desktop shell

  <https://wiki.gnome.org/Projects/GnomeShell>
  <https://gitlab.gnome.org/GNOME/gnome-shell/>

+ GNOME 3.6 集成输入法

  <https://help.gnome.org/misc/release-notes/3.6/i18n-ibus.html>

+ ibus-libpinyin: Intelligent Pinyin engine based on libpinyin for IBus

  <https://github.com/libpinyin/ibus-libpinyin>


## LICENSE

This repository is released under:
`LGPL-2.1-or-later` or `GPL-3.0-or-later`, at your option.

本仓库的许可从以下之中任选:
LGPL-2.1 或更新版本, GPL-3.0 或更新版本.
