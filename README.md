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
This can be in the environment settings in `.vscode/launch.json`, for example:

```json
    "env": {
        "PYTHONPATH": "${workspaceFolder}/.venv/lib/python3.13/site-packages"
    },
```

Alternatively, to ensure debug works consistently with VSCode UI too, I've found
that setting `lldb.launch.env` more relaible. See below.

The `launch.json` example runs the executable directly rather than using `cargo`
because the latter seems to tweak the environment and was fiddly initially, so
to keep it simple, I'm running direclty with LLDB after a running
`maturin develop`.

### Debugging using the in-editor Run and Debug code hover buttons

To use the Run button, the `.cargo/config.toml` needs to know `PYTHONPATH`.
Apparently this should be an absolute dir, but I have found it works with
relative paths so have committed my personal config file running against local
`.venv`, if you have any issues replace with absolute dir, i.e.:

```toml
[env]
PYTHONPATH = "<absolute path to this dir>/.venv/lib/python3.13/site-packages"
```

To run with the Debug hover button add the following to your `settings.json`:

```json
    "lldb.launch.env": {
        "PYTHONPATH": "${workspaceFolder}/.venv/lib/python3.13/site-packages"
    }
```
