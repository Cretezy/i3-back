# Upstream

This feature could easily be upstream to i3. This could be added as new option to the [`focus` command](https://i3wm.org/docs/userguide.html#_focusing_moving_containers), such as `focus last`. Example usage would be:

```
bindsym $mod+Tab focus last
```

# Stack-based switcher

(This is just an idea, and is likely not be possible to implement with i3's current mark binding system)

I believe it would be possible to store the history of focus to allow using the `i3-back switch` command multiple times in a row,
which would match how Alt+Tab works on other desktop environments.

Considerations:

- It would be nice to know when `$mod` is released, which would reset the history index. This limitation makes it likely not possible. Alternatively, use a delay.
