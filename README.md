# Rustodoro
A simple and configurable command-line pomodoro timer written in Rust.

## Usage
`rustodoro <COMMAND>`

## Arguments
- `work`                        Start a work timer
- `short-break`                  Start a short break timer
- `long-break`                   Start a long break timer
- `set-work-time [OPTIONS]`      Configure the work timer
  - Options:
    - `-m`, `--minutes` `<MINUTES>`  Minutes component of the work timer
    - `-s`, `--seconds` `<SECONDS>`  Seconds component of the work timer
- `set-short-break-time [OPTIONS]` Configure the short break timer
  - Options:
  - `-m`, `--minutes` `<MINUTES>`  Minutes component of the short break timer
  - `-s`, `--seconds` `<SECONDS>`  Seconds component of the long break timer
- `set-long-break-time [OPTIONS]` Configure the long break timer
  - Options:
    - `-m`, `--minutes` `<MINUTES>`  Minutes component of the long break timer
    - `-s`, `--seconds` `<SECONDS>`  Seconds component of the short break timer
- `set-pomodoros-to-long-break <POMODOROS_TO_LONG_BREAK>` Configure the amount of pomodoros (work stints) to complete for a long break
- `help`                         Print this message or the help of the given subcommand(s)
