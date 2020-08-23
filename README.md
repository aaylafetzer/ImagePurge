[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/)
[![Actions Status](https://github.com/aaylafetzer/ImagePurge/workflows/Build/badge.svg)](https://github.com/aaylafetzer/RustCloneGitProfile/actions)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/aaylafetzer/ImagePurge)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/aaylafetzer/ImagePurge)

# ImagePurge
A program to sanitize image files so they're safe to share. Currently only purges the [exif](https://en.wikipedia.org/wiki/Exif) data.

## Usage
This information can also be found with ``./impurge -h``
```
ImagePurge 1.0
Aayla Fetzer
Removes the exif data from image files

USAGE:
    impurge <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PATH>    Path to remove the exif data from. Only single files are supported.
```

## To do
- Add options to write custom metadata to the output files
- Facial detection algorithm to draw rectangles and hide faces