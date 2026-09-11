# Rabbyte 🐇

> **n.** The amount of lagomorph memory equal to exactly 8 rabbits. The pun is fully intended.

```
                    ,
                   /|      __
                  / |   ,-~ /
                 Y :|  //  /
                 | jj /( .^
                 >-"~"-v"
                /       Y
               jo  o    |
              ( ~T~     j
               >._-' _./
              /   "~"  |
             Y     _,  |
            /| ;-"~ _  l
           / l/ ,-"~    \
           \//\/      .- \
            Y        /    Y    🐇 8 rabbits = 1 Rabbyte 🐇
            l       I     !
            ]\      _\    /"\
           (" ~----( ~   Y.  )
        ~~~~~~~~~~~~~~~~~~~~~~~~~~
```

## The Joke

You know how **8 bits = 1 byte**?

Well, obviously **8 rabbits = 1 Rabbyte**. 🐇×8 = 🧠

`Rabbyte` is a deeply serious scientific tool that scans your system dictionary
(`/usr/share/dict/words`) and finds every word ending in **"bit"** — because every
word ending in "bit" is clearly a rabbit in disguise. Then it converts each one
into its byte-aligned equivalent: the trailing `bit` becomes `byte`.

- `Rabbit` becomes a **`Rabbyte`** 🐇
- `Habit` becomes a **`Habyte`**
- `Hobbit` becomes a **`Hobyte`**
- `Bit` becomes a **`Byte`** (the original, humble)

Why? Because the wordplay demanded it. We regret nothing. (8 rabbits = 1 Rabbyte. Tell your friends.)

## Example Output

```
🐇 RABBYTE 🐇
8 rabbits = 1 Rabbyte. Converting your dictionary, one hop at a time...

[1] Rabbit =( * 8 )=> Rabbyte
[2] Habit =( * 8 )=> Habyte
[3] Hobbit =( * 8 )=> Hobyte
[4] Bit =( * 8 )=> Byte
...
🐇>━━━━━━━━━━━━━━━━━━━━━━━━━━ 148,271/148,279 (99%) — 4 Rabbytes so far
```

(Colors not shown here — the word is green, the `=( * 8 )=>` is yellow, the
resulting Rabbyte is bright magenta, and the counter is cyan. In a real
terminal it looks *fabulous*.)

At the end you get the harvest report:

```
✨ Scan complete — the hutch has been harvested! ✨
Hopped through 148,279 words in 148.3ms and produced 4 Rabbytes (1 Rabbyte = 8 rabbits, obviously).
```

## Installation

**Linux only!** (See [Requirements](#requirements) below for why.)

You need a Rust toolchain — grab it via [rustup](https://rustup.rs) if you
don't have it, then:

```bash
git clone https://github.com/your-name/Rabbyte.git
cd Rabbyte
cargo build --release
```

Your freshly compiled rabbit-to-byte converter will be waiting in
`target/release/Rabbyte`.

## Usage

```bash
cargo run --release
# or, if you built it:
./target/release/Rabbyte
```

That's it. No flags, no config, no mercy. It reads `/usr/share/dict/words`,
hops through every word, and prints one glorious line per rabbit discovered.

## Requirements

- **Linux.** Rabbyte reads the system dictionary at `/usr/share/dict/words`,
  a path that only exists on Linux (and refuses to pretend otherwise).
- That file comes from a dictionary package. If it's missing, install it:
  - Debian/Ubuntu: `sudo apt install words`
  - Fedora: `sudo dnf install words`
  - Arch: `sudo pacman -S words`
  - Alpine: `sudo apk add words`

If the file is missing, Rabbyte prints a friendly error (with these exact
instructions) and exits — no crashes, just a sad empty hutch.

## How It Works

1. Counts the lines in the dictionary (quick first pass) to size up the progress bar.
2. Scans word by word, updating a live progress bar in real time.
3. Case-insensitively checks if a word ends in `bit`.
4. On a match, prints `[N] Word =( * 8 )=> Wordbyte` *through* the progress
   bar (`indicatif`'s suspend/println mechanism), so the bar stays pinned and
   pretty while joke lines scroll above it.
5. Prints a final summary of words scanned and Rabbytes produced.

## FAQ

**Q: Is this useful?**
A: It is *byte*-fully useful.

**Q: Where do the rabbits come from?**
A: `/usr/share/dict/words`. They live between `raa` and `rabbinical`.

**Q: Can I run it on macOS/Windows?**
A: You *could* try, but there's no `/usr/share/dict/words` there — the rabbits
refuse to travel. Install the dictionary, symlink it to that path, and the
bunnies might forgive you. Or just use Linux, like nature intended.

**Q: Why 8 rabbits?**
A: Because 8 bits make a byte. We didn't make the rules; we just obey them
religiously.

## License

MIT — do whatever you want, the rabbits are free-range. 🐇🐇🐇🐇🐇🐇🐇🐇 = 1 Rabbyte.
