# zathura-markdown-rs
This repository contains a plugin for [Zathura](https://pwmt.org/projects/zathura) which one day will be able to render markdown well.
The main purpose of this plugin is to provide a viewer which is independent of any text editor or IDE, and it has the benefit of being able to generate PostScript, PDF or SVG files from the rendered text.


## To Do
- [ ] Translate Markdown into [Pango Markup](https://developer.gnome.org/pango/stable/pango-Markup.html).
- [ ] Render parsed Pango Markup to the Cairo context.

## Usage

You can test it by running `make test`. Make sure you have Rust installed, as well as Zathura.

## Installation

Install by running `cargo build --release` and `sudo make install`.
