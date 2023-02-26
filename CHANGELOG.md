# v0.3.2 - 2023-02-26

- Added `--mark`/`-m` option to change i3 mark name (default is `_back`)
  - Also re-add `--debug`/`-d` option
- Lowered restart timeout from 1s to 0.5s

# v0.3.1 - 2023-02-26

- Fix support for floating windows
- Refactor error handling

# v0.3.0 - 2023-02-26

- Switch to using i3 marks. No longer needs D-Bus/config. Thanks [`@nicarran`](https://github.com/nicarran) for the [idea](https://github.com/i3/i3/issues/838#issuecomment-481531210)!
  - Please update your switch `bindsym` to the new command: `bindsym $mod+Tab [con_mark=_back] focus`
- Switch to optimized builds and use Debian container to build `deb` release

# v0.2.0 - 2023-02-26

- Switch to using D-Bus for IPC when switching. Config file is no longer needed

# v0.1.3 - 2023-02-26

- Added Debian/Ubuntu package

# v0.1.2 - 2023-02-26

- Fix crash when restarting i3 while execution
- Handle exiting program by clearing config's `last_focused_id`
- Improve algorithm to find focused window from i3 tree. Thanks [`@lbonn`](https://github.com/lbonn) for [`i3-focus-last`](https://github.com/lbonn/i3-focus-last)!

# v0.1.1 - 2023-02-25

- Added AUR package

# v0.1.0 - 2023-02-25

- First release of i3-back!
