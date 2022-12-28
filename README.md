# zk-spaced

Spaced repetition memoization for [zk](https://github.com/mickael-menu/zk).


## Setup

Install the `zk-spaced` binary in your `PATH` and add an alias either to your
`~/.config/zk/config.toml` or to the notebook specific configuration:

```toml
[alias]
review = "zk list --quiet --tag sr --format json | zk-spaced"
```

You can adjust the tag to match all notes/cards you want to learn. Of course you
need to tag the notes with that tag.
