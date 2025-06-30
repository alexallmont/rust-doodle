# Rust Doodle

General experiments in day-to-day work

## Nalgebra and faer to PyArray conversion

### General strategy

- Use `ArrayView2::from_shape_ptr` respecting shape/stride of underlying data.
- Nalgebra and faer differ in a few methods, need to be made into common code.

### Setup and debugging

Create a venv and add required libs. I initially ran with `uv` (and `pipx`/
`uv tool`for maturin) but hit some linkage issues and missing packages, so for
now I'm playing it safe and doing it all through regular old `venv`:

```sh
python3 -m venv .venv
source .venv/bin/activate
pip install --upgrade pip setuptools wheel maturin numpy
```

Note that this is a **binary** not a lib as is usual with PyO3 projects, and as
such you cannot `cargo run` because `maturin develop` is geared towards the
development of modules and not executables like this. If you want to run from
the command line then prefix with `PYTHONPATH`:

```sh
PYTHONPATH=.venv/lib/python3.13/site-packages cargo run
```

Similarly, this extra environment information is required to launch for debug.
See `.vscode/launch.json` for example. This launches directly rather than
running with `cargo` as the latter seems to tweak the env; it's better to use
LLDB directly.