# GTA:SA Linux Trainer
This is a very simple work in progress cli trainer for GTA San Andreas 1.0 running under Wine/Proton written in Rust.

This project was created for educational purposes, as I was learning how to reverse engineer games using [PINCE](https://github.com/korcankaraokcu/PINCE/tree/master) and [gdb](https://www.gnu.org/software/hurd/gdb.html).

I wanted to make a Rust version of [my original C++ project](https://github.com/Rikaisan/gta-sa-linux-trainer) and thanks to [clap](https://github.com/clap-rs/clap) and other Rust features adding new functionality is a breeze.

# Considerations
1. Make sure GTA:SA is downgraded to version 1.0, you can do it using the [GTA SA Open Downgrader](https://github.com/xxanqw/gtasa-open-downgrader)
2. Make sure the game is NOT modded
3. Make sure to disable ALSR for the game by setting the steam/wine/proton launch options to: `setarch x86_64 -R %command%`