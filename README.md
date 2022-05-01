# Uklid 🧹

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/vacekj/uklid/Continuous%20integration)
![Crates.io](https://img.shields.io/crates/v/uklid)
![Crates.io](https://img.shields.io/crates/d/uklid)
[![codecov](https://codecov.io/gh/vacekj/uklid/branch/master/graph/badge.svg?token=6j0NfrmyJG)](https://codecov.io/gh/vacekj/uklid)

Uklid (pronounced `[uːklɪt]`, "cleanup" in Czech) is an interactive CLI `node_modules` cleaner written in Rust.
It looks for top-level node_modules in your computer, shows you their size and allows you to delete them conveniently.

## Installation
```bash
cargo install uklid
```

## Usage
```bash
uklid 0.2.0
Atris <vacekj@outlook.com>
Interactively delete node_modules.

USAGE:
    uklid [OPTIONS]

OPTIONS:
    -d, --dry            Don't delete anything, only print found directories
    -h, --help           Print help information
    -p, --path <PATH>    Path to start recursive search for node_modules from
    -V, --version        Print version information

```

## Features
- recursively search your home directory for `node_modules`
- limit search to a specific directory
- show sizes
- multi-select what you want to delete
- show total storage freed up
- non-interactive mode (`uklid --help`)

## Roadmap
- better searching algorithm
- ability to not compute sizes to drastically speed up searching
- async mode?
- tests with 100% coverage
  - codecov reporting with readme badge
- multi-threaded searching and cleaning
- benchmarks against js-based alternatives using [https://github.com/sharkdp/hyperfine](hyperfine)