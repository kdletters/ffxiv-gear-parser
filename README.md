# FFXIV Share Parser (Rust)

A Rust implementation of the FFXIV gearset share string parser, translated from the original TypeScript version from [ffxiv-gearing](https://github.com/Asvel/ffxiv-gearing).

## Overview

This library can parse base62-encoded gearset share strings used by FFXIV gearing tools. It decodes equipment configurations including:

- Job and job level information
- Equipment IDs and materia configurations
- Custom stat configurations for crafted gear
- Sync level information for level-synced content

## Run

```commandline
cargo run -- 45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ffxiv-gear-parser = { path = "." }
```