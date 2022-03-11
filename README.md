# Wordle Helper

Rust implementation of a [Wordle](https://www.nytimes.com/games/wordle/index.html) helper - never see that dreaded `X/6` again!

## Prerequisites

Install rust with the following from the [Rust Website](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Running

Clone the repo:

```
git clone git@github.com:grahamkeenan/wordle-helper-rs.git
```

Run the project with:

```
cargo run --release
```

## Usage

The project takes a list of most 5-letter words (`corpus.txt`) and filters them down based on the answers you give the program.

First off you enter your guess:

```
Enter guess:
cheap
```

The program will then ask you to enter the result from Wordle. The results you enter are either `x`, `p`, or `c`. `x` represents that the character is not present in the word (black box from Wordle), `p` represents that the letter is present but in the wrong position (yellow box from Wordle), and `c` represents that the character is present and in the correct position (green box from Wordle).

```
Enter Result (x = not present, p = present in wrong position, c = correct position):
xxpcx
```

The word list is then filtered based on the result information and displays the words that match the criteria.

Eventually the list will be filtered down until one word remains or you enter a word correctly in Wordle itself.

Never see the `X/6` again!

```
60% of the time, it works every time.
```