# Upstream

This feature could easily be upstream to i3. This could be added as new option to the [`focus` command](https://i3wm.org/docs/userguide.html#_focusing_moving_containers), such as `focus last`. Example usage would be:

```
bindsym $mod+Tab focus last
```

# Use socket instead of config

Instead of using a config to store the last focused window's ID, the `i3-back switch` command could send a message on a socket to switch,
which the server would handle. This would remove the need for file I/O, but would increase complexity (sockets can be difficult to setup/managed).

# Stack-based switcher

(This is just an idea, and may not be possible to implement with i3's current binding system.)

I believe it would be possible to store the history of focus to allow using the `i3-back switch` command multiple times in a row,
which would match how Alt+Tab works on other desktop environments.

How this could work:

- In `i3-back start` (the daemon), store a history of all window switches (up to a limit)
  - Each items should be unique and be sorted by recency
- When doing `i3-back switch`, store in a flag in the config file that the upcoming switch should not be recorded
  - It should then use the currently focused window ID to find the next item to focus on
  - It should verify that it goes to the next existing window. For example, given window A, B, C in the history, if B was closed, it should go from A to C
- When `i3-back start` receives a focus switch event, it should check if the current config to see if it should update it
  - If it was triggered by `i3-back switch`, indicated by the flag, it should not update the config

Considerations:

- It would be nice to know when `$mod` is released, which would reset the history index. This limitation makes it likely not possible. Alternatively, use a delay.
