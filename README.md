# i3-back

An [i3](https://i3wm.org/) utility to switch focus to your last focused window. Allows for behavior similar to Alt+Tab on other desktop environments.

**Features**:

- Can switch between your 2 most recent windows
  - Runs daemon to watch for window focus changes and records to a config file
- Supports floating windows
- Can be binded to any key through i3's `bindsym`

## Demo

TODO

## Installation

i3-back is written in Rust. It can be installed through many method:

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), Rust's package manager

  ```
  cargo install i3-back
  ```

- [GitHub Releases](https://github.com/Cretezy/i3-back/releases), as binary

### Setup

In your i3 configuration (`~/.config/i3/config`):

```
# Start the daemon which listens to focus changes and records it to ~/.config/i3-back/config.toml
exec --no-startup-id ~/.cargo/bin/i3-back start

# Bind a switch key, which focuses the previously focused window
bindsym $mod+Tab exec ~/.cargo/bin/i3-back switch
```

Replace `~/.cargo/bin` with wherever the i3-back binary is placed if not installed through Cargo.
