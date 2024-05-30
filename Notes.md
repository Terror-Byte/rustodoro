# General Gist
Print current time, print progress bar? How to work out the percentage?

If current percentage:
0% - 9%: no bars
10% - 19%: 1 bar
20% - 29%: 2 bars
30% - 39%: 3 bars
40% - 49%: 4 bars
50% - 59%: 5 bars
60% - 69%: 6 bars
70% - 79%: 7 bars
80% - 89%: 8 bars
90% - 99%: 9 bars
100%: 10 bars (finished)

[==========] 100%
100% [==========]

5 mins remaining.
85% [========  ]
[========  ] 85%

Here's the format we want:
5 mins remaining.
[========  ] 85%

Do we want to save completed timer things to a file?
What do we want to load from a config? (json, yaml, toml even?)
We want these to be configurable from the commandline too!

rustodoro <- just run the work timer if we do this?
rustodoro --work
rustodoro --short-break
rustodoro --long-break
rustodoro --set-work-time --minutes 25
rustodoro --set-work-time --seconds 1500
rustodoro --set-short-break-time --minutes 5
rustodoro --set-short-break-time --seconds 300
rustodoro --set-long-break-time --minutes 15
rustodoro --set-long-break-time --seconds 900
rustodoro --set-pomodoros-till-long-break 4
rustodoro --set-display-in-secs true (if true, display timer in seconds. Otherwise, display in minutes)

VIM Note: This is early days, Liam! There's going to be some teething issues/friction initially but you're going to learn as you go. You got this! You can learn this!

How do I do colours when writing to terminal?

How do we figure out percentages again?
(50/100)*100 = 50
(val/total)*100 = percentage

How to do the progress bar...
10 spaces in total.
Divide current number by 10?

55/10 = 5
5 =, 10 - G5 spaces. 

WAIT, I have this the wrong way round.

Add in commands to configure!
When we finish a work stint, print out that we can have a break now + how many work stints we've taken?

https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

1 arg: no arguments passed (first arg is the binary name).
2 args: --work, --short-break, --long break, --help. If none of these, exit out cause unrecognised.
3 or 4 args: --set-work-time, --set-short-break-time, --set-long-break-time, --set-pomodoros-to-long-break, --set-display-in-secs

--help -> what info do I want displaying?
SET NERD FONT.

Maybe find a crate to handle this at some point? Figuring it out myself is fun for now though!