# TODO

- Add brightness setting (0-100) to allow brightening dark images
- Instead of using 2d vectors throughout the program, flatten them into single vectors
- Speed up GIF encoding (seems to take up the most time when converting GIFs)

## Bugs

- `--invert` causes a weird reordering of the `--char-override` value (RASCII issue).

## Crates Checklist

- More robust error handling. Make it so we can avoid all panics!
    - In this process, we need to create our own errors which must be meaningful and useful.
    - Probably should remove the "Internal" error since it's vague.

