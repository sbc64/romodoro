# Romodoro
Rust pomodoro timer with custom sounds in each interval

## Goals

Use custom sounds to better train myself to begin work.

## Config

```toml
#~/.config/romodoro.toml
order = ["BeginWork", "ShortBreak", "BeginWork", "LongBreak"]

[long_break]
duration = 600
sound = "/home/sebas/repos/programs/romodoro/src/bowl.mp3"

[short_break]
duration = 300
sound = "/home/sebas/repos/programs/romodoro/src/bowl.mp3"

[begin_work]
duration = 1800
sound = "/home/sebas/repos/programs/romodoro/src/bojack.mp3"


```
