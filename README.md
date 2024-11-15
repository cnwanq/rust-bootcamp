# RUST BOOTCAMP - Rust 训练营

## 简介

这是一个 Rust 训练营的项目，旨在帮助初学者掌握 Rust 编程语言的基础知识和实践。

## 目标

- 了解 Rust 编程语言的基本概念和语法。
- 掌握 Rust 的基本数据类型和控制流。
- 编写简单的 Rust 程序。
- 解决常见的 Rust 编程问题。
- 学习 Rust 的标准库和常用的第三方库。

## 环境设置

### 安装 rust

请按照 [Rust 官方文档](https://www.rust-lang.org/zh-CN/tools/install) 的说明安装 Rust。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 VSCode 插件

- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Better Comments: 优化注释显示
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Github Copilot: 代码提示
- indent-rainbow: 缩进显示优化
- Prettier - Code formatter: 代码格式化
- REST client: REST API 调试
- rust-analyzer: Rust 语言支持
- Rust Test lens: Rust 测试支持
- Rust Test Explorer: Rust 测试概览
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化
- YAML: YAML 文件支持

### 安装 pre-commit

pre-commit 是一个代码检查工具，可以在提交代码前进行代码检查。

> 安装成功后运行 `pre-commit install` 安装 pre-commit 钩子。

### 安装 Cargo deny

Cargo deny 是一个 Cargo 插件，可以用于检查依赖的安全性。

```bash
cargo install --locked cargo-deny
```

### 安装 typos

typos 是一个拼写检查工具。

```bash
cargo install typos-cli
```

### 安装 git cliff

git cliff 是一个生成 changelog 的工具。

```bash
cargo install git-cliff
```

### 安装 cargo nextest

```bash
cargo install cargo-nextest --locked
```
