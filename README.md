# Rust Doodle

General experiments in day-to-day work

## Nalgebra and faer to PyArray conversion

### General strategy

- Use `ArrayView2::from_shape_ptr` respecting shape/stride of underlying data.
- Nalgebra and faer differ in a few methods, need to be made into common code.

### Setup and debugging

Create a venv, add required libs and set up developer enviroment by running
`./setup.sh`. This uses regular pip for tooling after I hit some linkage issues
running `uv` and `pipx` for tooling, so I'm playing it safe but your mileage
may vary.

The setup script runs the package setup below, and also patches
`.cargo/config.toml` and `.vscode/settings.json` to make the integrated editor
debugging experience nicer (see notes below).

```sh
python3 -m venv .venv
source .venv/bin/activate
pip install --upgrade pip setuptools wheel maturin numpy
maturin develop
```

Note that crate is a **binary** test app and not a lib as is usual with
`maturin develop` projects. This seemed to cause problems when using `cargo run`
and I had to manually specify the `PYTHONPATH` before running. This now seems
to work OK for me on certain machines but if you hit problems, you can specify
on the command line with something like:

```sh
PYTHONPATH=.venv/lib/python3.13/site-packages cargo run
```

Similarly, this extra environment information is required to launch for debug.
This can be in the environment in `.vscode/launch.json` or via the neater editor
integration detailed below. If you hit problems, specify the env in
`launch.json` like this.

```json
    "env": {
        "PYTHONPATH": "${workspaceFolder}/.venv/lib/python3.13/site-packages"
    },
```

Alternatively, to ensure debug works consistently with VSCode UI too, I've found
that setting `lldb.launch.env` more relaible, as when set I do not need to
modify environment variables in `launch.json`. See below.

### Fixing up site packages

This workspace was built using my local `python3.13` so if you are using a
different python version, you need to grep and replace the site packages in
the demonstration `.vscode/launch.json` and `.cargo/config.toml` files. This is
done automatically when you set up the environment with `./setup.sh`.

The `launch.json` example runs the executable directly rather than using `cargo`
because the latter seems to tweak the environment and was fiddly initially, so
to keep it simple, I'm running direclty with LLDB after a running
`maturin develop`.

### Nicer integrated editor debugging using VSCode's Run and Debug hover buttons

VSCode has a hover button over code and tests that says 'Run | Debug'. These
work fine for vanilla rust or vanilla python projects but I've found to work
correctly with PyO3/maturin projects like this I had to do some extra steps
detailed here (but if you ran './setup.sh` this is done automatically).

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
