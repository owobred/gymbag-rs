# ``gymbag-rs``
![gymbag](https://cdn.discordapp.com/emojis/1067809504919568515.webp?size=96&quality=lossless)  :trollface:

## What is this?
Silly fast decoding for `gymbag2` files, written in the crab language. 🦀

## Why does this exist?
I wanted to mess about with [`pyo3`](https://github.com/PyO3/pyo3), to get a better grip with rust-python ffi. This isn't currently used anywhere, mostly due to being (a) a bit of a meme and (b) being a pain to maintain when compared to the easily modifiable [python version](https://github.com/VedalAI/neuro-amongus) 💖. This provides a *major* (>50x) speedup when compared to the python version, but only when converting from `.gymbag2` to a pickle file of x data and y data. If this data is passed directly to python (over the ffi boundary), there speed is comparable to the python version directly loading the pickle files.

## How do I use this?
You probably shouldn't. I haven't tested it beyond making sure it runs, and for all I know it may be outputting completely useless data.

However, if you really want to, you'll need the following: (or at least, this is what I used)
 - Cargo & the rust toolchain (I used `cargo 1.71.0-nightly`)
 - Python 3.11 (I used `Python 3.11.0rc1` for no good reason other than it being the latest one I had installed)
 - Protoc (I used `libprotoc 23.1`)
 - Poetry (I used `poetry 1.4.2`)
 - Some `.gymbag2` files to decode

Once you have all of these, run `poetry install` in the root of this git directory. This should setup a virtual environment & install the required libraries.
From here, run `poetry shell` to enter into the virtual environment.
This then allows you to access `maturin`, which interfaces with `cargo` to build the library and bindings.
To build a python wheel and automatically install it into the venv, run `maturin develop --release`[^1].

After doing this, the library should be importable as `gymbag_rs`, and provides a few wrappers around the rust functions that load gymbags, all of which follow the naming style of `read_<...>`. These take paths to gymbag files, and give useful data.

```python
from gymbag_rs import read_many_gymbag_to_xs_ys

# This provides a speedup over repeatedly calling `read_gymbag_to_xs_ys` for each file
# This is done by using `rayon` to multithread the reading and parsing steps on the rust
# side of things, before passing over the processed data.
xys = read_many_gymbag_to_xs_ys([file for file in os.listdir("./recordings/") if file.endswith(".gymbag2")])

# This gives you sets of x and y data, in overlapping windows of 10 gamestates.
for (x_sets, y_sets) in xys:
    do_something_with_x_sets(...)
    do_something_with_y_sets(...)
```

## Issues with this implementation
 - Errors are not handled by the rust side in any way shape or form. If something goes wrong, the library will panic and throw a `pyo3_runtime.PanicException`. Whilst these are catchable, this isn't really a usable way of being.
 - It is a complete mess. I struggled to understand sections of the original python code, and some of my attempts to implement things in rust may not be equivalent to hte original python code.
 - The `build.rs` is a clusterfuck. I hate `prost` when it doesn't work, and love it when it does.
 - This suffers from an annoying bug with pyo3 and venvs that forces a recompile when `cargo clippy` is ran, which is a major part of rust-analyser. This will make development a bit annoying.
 - This was made for fun. There are likely many things that don't conform to best practices.
 - Rust `Vec`s serialize really slowly into python `lists`, which causes importing these to stall really badly. Due to the GIL, this is only single threaded, through this can sort-of be worked arround by using the `multiprocessing` package to start multiple batches of deserialization, which all have their own GIL and thus can go from rust -> python across multiple cores.

## If you're trying to update this
 - The protobuf files are different, to get `prost` to compile them, I shoved in `package gymbag.proto` to every file. This means they build, but are different to the original.
 -  The original protobuf files often contained ids as `uint32`s, which were later set to -1 in python code to represent missing / invalid values. This wasn't possible with the way the files were built in rust (you can't have a negative `u32`), so I manually changed any problematic types to use `sint32`. 

[^1]: While you can run this without using the `--release` flag, it will take a bit longer to deserialize gymbag files and run through gamestates.