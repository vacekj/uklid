# Uklid 🧹

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/vacekj/uklid/Continuous%20integration)
![Crates.io](https://img.shields.io/crates/v/uklid)
![Crates.io](https://img.shields.io/crates/d/uklid)

Uklid (pronounced `[uːklɪt]`, "cleanup" in Czech) is an interactive CLI `node_modules` cleaner written in Rust.
It looks for top-level node_modules in your computer, shows you their size and allows you to delete them conveniently.

## Features
- recursively search your home directory for `node_modules`
- limit search to a specific directory
- show sizes
- multi-select what you want to delete
- show total storage freed up

## Roadmap
- tests with 100% coverage
- multi-threaded searching and cleaning
- accept command-line arguments (non-interactive mode)
- a GUI