# Stack-based switcher

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
