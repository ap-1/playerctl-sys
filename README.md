# playerctl-sys

[![Version](https://img.shields.io/crates/v/playerctl-sys)](https://crates.io/crates/playerctl-sys)
[![License](https://img.shields.io/crates/l/playerctl-sys)](https://github.com/ap-1/playerctl-sys/blob/main/LICENSE)

Unsafe Rust bindings to libplayerctl. This crate makes use of the [playerctl](https://github.com/altdesktop/playerctl) source, which is licensed under LGPL-3.0. 

The major and minor version numbers of this crate should match those of [playerctl](https://github.com/altdesktop/playerctl). The patch number will vary. If this is not the case, it is likely that this crate is out of date; please create an issue. 

Some API members require types from [glib-sys](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib_sys/index.html), which are also exported from this crate for convenience. Note that the [glib](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/index.html) crate re-exports `glib-sys` as `glib::ffi`.

## Relevant docs
- [This crate's docs](https://ap-1.github.io/playerctl-sys/playerctl_sys/index.html)
- [Playerctl reference manual](https://dubstepdish.com/playerctl/index.html)
- [glib-sys crate docs](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib_sys/index.html)
- [GLib docs](https://docs.gtk.org/glib/index.html)