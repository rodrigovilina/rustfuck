# Rustfuck

**Rustfuck** is a Rust implementation of the **Brainfuck** esoteric programming
language.

Brainfuck was designed to have a small implementation and run on a simple
virtual machine.

In Brainfuck you have a memory of `30_000` bytes and a data pointer to point to
any which of these bytes. There are exactly eight instructions, each exactly one
character long.

- `+`: Increment the currently pointed byte by one. If the current value is
  `255` it'll wrap to `0`.
- `-`: Decrement the currently pointer byte by one. If the current value is `0`
  it'll wrap to `255`.
- `>`: Increment the data pointer by one. If the current value is `29_999` it'll
  wrap to `0`.
- `<`: Decrement the data pointer by one. If the current value is `0` it'll wrap
  to `29_999`.
- `.`: Writes the currently pointed byte to STDOUT.
- `,`: Reads exactly one byte from STDIN and store it in the currently pointed
  byte.
- `[`: Run the inner instructions only if the currently pointed byte is
  non-zero.
- `]`: Return to the matching left bracket if the currently pointed byte is
  non-zero.

Any other characters will be ignored.

## Tools

For now, the only available tools is a **file runner**.

### Runner

Run a file of brainfuck code just by referencing it by name:

```sh
rustfuck hello_world.bf
```

After running the file, the whole state of the machine will be printed to
screen. Memory cells with a value of zero are not presented.
