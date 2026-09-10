# game-params

Genearates params for Steam

## all available options (example)

```toml
# Launch options for the game/executable
flags = [
    "-norestrictions",
    "-nomemrestrict"
]

# Any key + value can go here
[env]
vblank_mode = "0"
MESA_VK_WSI_PRESENT_MODE = "immediate"

[gamescope]
# This is equivalent to '-w 1920 -h 1080' in gamescope
width = 1920
height = 1080
scaler = "stretch"
fullscreen = true
flags = [
    "--force-grab-cursor",
    "--immediate-flips",
]

# Wine DLL overrides
[dll-overrides]
dinput8 = "n,b"
```

## usage

```
Usage: game-params <FILE>

Arguments:
  <FILE>

Options:
  -h, --help     Print help
  -V, --version  Print version
```
