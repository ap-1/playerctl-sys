# playerctl-sys

[![Version](https://img.shields.io/crates/v/playerctl-sys)](https://crates.io/crates/playerctl-sys)
[![License](https://img.shields.io/crates/l/playerctl-sys)](https://github.com/ap-1/playerctl-sys/blob/main/LICENSE)

Unsafe Rust bindings to libplayerctl. This crate makes use of the [Playerctl](https://github.com/altdesktop/playerctl) and [GLib](https://gitlab.gnome.org/GNOME/glib/) sources, which are licensed under LGPL-3.0. Some API members require types from [glib-sys](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib_sys/index.html). Note that the [glib](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/index.html) crate re-exports `glib-sys` as `glib::ffi`.

## Dependencies

You must have [**clang**](https://rust-lang.github.io/rust-bindgen/requirements.html) and [**glib-2.0**](https://gitlab.gnome.org/GNOME/glib/-/blob/main/INSTALL.md) installed on your system to build this crate. In addition, you must either have [**playerctl**](https://github.com/altdesktop/playerctl/#installing) or [**meson**](https://github.com/mesonbuild/meson#installing-from-source) installed. If playerctl is not found, meson will be used to build it from source. It is strongly recommended to install playerctl as that will take care of both the glib-2.0 and the meson/playerctl requirement.

## Relevant documentation

- [playerctl-sys crate docs](https://docs.rs/playerctl-sys)
- [glib-sys crate docs](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib_sys/index.html)
- [Playerctl reference manual](https://dubstepdish.com/playerctl/index.html)
- [GLib docs](https://docs.gtk.org/glib/index.html)
