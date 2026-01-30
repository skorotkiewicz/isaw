# isaw

> Generate every letter combination. Find any word.

A fast, elegant CLI tool for generating permutations, combinations, and searching through them.

## Install

```bash
cargo install --path .
```

## Usage

### Permutations

Generate all arrangements where order matters:

```bash
isaw permutations abc
# a, b, c, ab, ac, ba, bc, ca, cb, abc, acb, bac, bca, cab, cba

isaw permutations cat --search "at"
# at, cat, atc  ← highlighted matches
```

### Combinations

Generate selections where order doesn't matter:

```bash
isaw combinations abcde --length 3
# abc, abd, abe, acd, ace, ade, bcd, bce, bde, cde
```

### Words

Generate all possible words from letters (Scrabble-style):

```bash
isaw words hello --search "ell" --unique
```

### Search

Search through custom alphabet combinations:

```bash
isaw search "cat" --letters "aeioutcsm" -n 3
# Finds: cat

isaw search "^a.*z$" --regex -n 4
# Regex pattern matching
```

### Count

Preview totals without generating:

```bash
isaw count abcdef
# Length 1: 6
# Length 2: 30
# Length 3: 120
# ...
# Total: 1956
```

## Options

| Flag | Description |
|------|-------------|
| `-m, --min` | Minimum length |
| `-x, --max` | Maximum length |
| `-s, --search` | Filter by pattern |
| `-i, --ignore-case` | Case insensitive |
| `-r, --regex` | Use regex patterns |
| `-u, --unique` | Deduplicate results |
| `-n, --length` | Exact length |

## License

MIT
