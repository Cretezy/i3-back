# i3-back

An [i3](https://i3wm.org/)/[Sway](https://swaywm.org/) utility to switch focus to your last focused window. Allows for behavior similar to Alt+Tab on other desktop environments.

**Features**:

- Can switch between your 2 most recent windows
  - Runs a daemon (background process) to listen for focus changes and sets a mark
- Supports i3 and Sway
- Supports floating windows
- Can be binded to any key through i3's `bindsym`

## Demo

https://user-images.githubusercontent.com/2672503/221384419-3d62413d-8987-4147-82bc-5e87cea8bb90.mp4

## Installation

i3-back requires i3/Sway. It is written in Rust. It can be installed through many method:

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Rust's package manager) package:

  ```
  cargo install i3-back
  ```

- Arch Linux from the AUR ([i3-back-bin](https://aur.archlinux.org/packages/i3-back-bin)):

  ```
  yay -S i3-back-bin # Or with paru or other AUR wrappers
  # Or manually: https://wiki.archlinux.org/title/Arch_User_Repository#Installing_and_upgrading_packages
  ```

- Debian/Ubuntu as a `deb` from [GitHub releases](https://github.com/cretezy/i3-back/releases)

- Binary from [GitHub releases](https://github.com/cretezy/i3-back/releases)

### Setup

In your i3/Sway configuration (`~/.config/i3/config`/`~/.config/sway/config`):

```
# Start the daemon which listens to focus changes and sets _back mark
exec --no-startup-id ~/.cargo/bin/i3-back

# Bind a switch key, which focuses the previously focused window
bindsym $mod+Tab [con_mark=_back] focus
```

Replace `~/.cargo/bin` with wherever the i3-back binary is placed if not installed through Cargo.

## How it works

The daemon (`i3-back`) listen for i3 window events. When a window event is received, i3-back records the previously focused window ID. When the previous focus has changed, it sets the `_back` [mark](https://i3wm.org/docs/userguide.html#vim_like_marks).

When the switch bind is called, it focuses the mark set by the daemon.

## Prior work

- [`@nicarran`](https://github.com/nicarran)'s [snippet](https://github.com/i3/i3/issues/838#issuecomment-481531210)
- [`@lbonn`](https://github.com/lbonn)'s [`i3-focus-last`](https://github.com/lbonn/i3-focus-last)
