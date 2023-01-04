# zk-spaced

Spaced repetition memoization for [zk](https://github.com/mickael-menu/zk).


## Setup

Install the `zk-spaced` binary in your `PATH` and add an alias either to your
`~/.config/zk/config.toml` or to the notebook specific configuration:

```toml
[alias]
review = "zk list --quiet --tag review --format json | zk-spaced"
```

You can adjust the tag to match all notes/cards you want to learn. Make sure to
tag the notes.


## Usage

Once you run `zk review` for each note that is due, you will see a panel with
the title. Think about the fact you want to remember and press <kbd>s</kbd> to
reveal the body of the note. Based on your own estimate press a number between
<kbd>0</kbd> and <kbd>5</kbd> to rate yourself from total blackout to perfect
recall.
