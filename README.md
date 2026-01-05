# ccwc

A Rust implementation of the Unix `wc` command.

## Usage

```
ccwc [OPTIONS] [FILE]
```

Options:
- `-l` - count lines
- `-w` - count words
- `-c` - count bytes
- `-m` - count characters

Reads from stdin if no file is provided.

## Reference

Based on [Coding Challenges - wc](https://codingchallenges.fyi/challenges/challenge-wc)
