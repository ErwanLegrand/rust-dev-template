# Rust Dev Template

Complete Rust development environment with all essential tools integrated.

## Purpose

This template provides a fully configured local Rust development environment that combines a base project structure, a dev-container for hermetic builds, pre-commit hooks for automated quality gates, and XTask-driven workflow commands. It is intended to be the starting point for any new Rust crate in the repository, eliminating the per-project setup of formatting, linting, testing, and editor tooling.

## Features

- **Base Project Structure** - Essential Rust patterns and error handling.
- **Development Container** - Isolated, consistent development environment with pre-installed tools.
- **Code Quality Tools** - Pre-commit hooks for automated formatting, linting, and testing.
- **Development Tasks** - XTask commands for a unified development workflow (`cargo xtask check`, `cargo xtask lint`, etc.).
- **VS Code Integration** - Optimized editor configuration, extensions, and debug support.
