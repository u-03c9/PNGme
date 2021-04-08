# PNGme

A CLI tool to encode and decode messages in a PNG file.

### Reference

This project was done by following a [tutorial](https://picklenerd.github.io/pngme_book/introduction.html) written by [picklenerd](https://github.com/picklenerd). This is my favorate tutorial so far. They didn't write a source code for me to copy, but rather:

- They explained the process of doing the project and cutting it down into pieces.

- They referenced the [PNG file structure spces](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html), so i can implement it.

- They provided some hints; like, which function you might need or an extra crate (library) that might help.

- But, the most important part is that they provided Unit tests for the majority of the project. So, I can make sure what I have built so far is working correctly. It is really the my favorait part of this tutorial.

### Dependancy

- [crc crate](https://crates.io/crates/crc) ([repo](https://github.com/mrhooray/crc-rs)) for CRC-32 sumcheck.
- [clap crate](https://crates.io/crates/clap) ([repo](https://github.com/clap-rs/clap)) for an easier command line arguments parsing.
