# chuni-discord-rpc

![CHUNIHM](https://img.shields.io/badge/CHUNITHM-X--VERSE--X_(v2.45)-purple)
![Licence](https://img.shields.io/github/license/pitiedwzr/chuni-discord-rpc)

Discord Rich Presence for CHUNITHM

# Features
- Display details like song information, playing status.
- Easy to configure for different version (Check [document](docs/memory_pointers.md) for more detail).

# Install

1. Go to the [Release page](https://github.com/Pitiedwzr/chuni-discord-rpc/releases/latest) and download `chuni-discord-rpc.exe`, put it inside the `game/bin` (same folder with your `launch.bat` or `start.bat`)

2. Add this line in your `launch.bat` or `start.bat` **before** `inject_x86.exe` is called:

```
start /min .\chuni-discord-rpc.exe
```

3. Start the game as usual and enjoy!

# Build

```
git clone https://github.com/Pitiedwzr/chuni-discord-rpc.git
cd chuni-discord-rpc
cargo build --release
```

# Licence

Distributed under the [MIT Licence](LICENSE).

The copyright of the art assets used for Discord Application belongs to SEGA, ~~SEGA PLEASE DON'T KILL ME~~.