This is a [rnote](https://github.com/flxzt/rnote) thumbnailer (forked from [`gnome-raw-thumbnailer`](https://gitlab.gnome.org/World/gnome-raw-thumbnailer)).

It will be used by Nautilus and other application supporting
thumbnailers to generate thumbnails from `.rnote` files.

It is built in Rust and uses the `rnote-engine` crate.

# Installation

## From Source

Requires cargo and meson to be installed.

`meson setup build && cd build && meson configure -Dprofile=release && meson install`

## Pre compiled binary

1. Download the [latest release artifact](https://github.com/ayykamp/rnote-thumbnailer/releases/latest) and place it somewhere in your PATH.
2. Run (you may need to create the necessary directories) \
  `cp data/com.github.flxzt.rnote.xml ~/.local/share/mime/packages/` \
  `cp data/rnote.thumbnailer ~/.local/share/thumbnailers/`

Note: If youre using nautilus, restart it with `nautilus -q` to generate thumbnails.

# Packaging

There is a `rnote-thumbnailer.spec` file as an example of RPM
packaging.  While it was written for Fedora it is probably not up to
it standards (it uses vendora tarball). It was written to allow build
a RPM package for deployment. You can use it by putting the source
tarball in `~/rpmbuild/SOURCES` and then `rpmbuild -ba rnote-thumbnailer.spec`.
