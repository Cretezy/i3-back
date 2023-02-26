# i3-back

An [i3](https://i3wm.org/) utility to switch focus to your last focused window. Allows for behavior similar to Alt+Tab on other desktop environments.

**Features**:

- Can switch between your 2 most recent windows
  - Runs daemon to watch for window focus changes and records to a config file
- Supports floating windows
- Can be binded to any key through i3's `bindsym`

## Demo

https://user-images.githubusercontent.com/2672503/221384419-3d62413d-8987-4147-82bc-5e87cea8bb90.mp4

## Installation

i3-back is written in Rust. It can be installed through many method:

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), Rust's package manager

  ```
  cargo install i3-back
  ```

- AUR - [i3-back-bin](https://aur.archlinux.org/packages/i3-back-bin), for Arch Linux

  ```
  yay -S i3-back-bin
  # or
  paru -S i3-back-bin
  # or any AUR wrapper, or manually: https://wiki.archlinux.org/title/Arch_User_Repository#Installing_and_upgrading_packages
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
