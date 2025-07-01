#/usr/bin/env bash
echo Creating environment in .venv...
python3 -m venv .venv
source .venv/bin/activate

echo Patching site-packages in develeper config...
SITE_PACKAGES=$(python -c "import sysconfig; print(sysconfig.get_path('purelib'))")

# VSCode Run in editor/general cargo config
mkdir -p .cargo
cat > .cargo/config.toml <<EOF
[env]
PYTHONPATH = "$SITE_PACKAGES"
EOF

# VSCode Debug in editor settings
mkdir -p .vscode
cat > .vscode/settings.json <<EOF
{
    "python.defaultInterpreterPath": "\${workspaceFolder}/.venv/bin/python",
    "lldb.launch.env": {
        "PYTHONPATH": "$SITE_PACKAGES"
    },
    "rust-analyzer.testExplorer": true
}
EOF

echo Set up remote and local pip dependencies...
pip install --upgrade pip setuptools wheel maturin numpy
maturin develop

echo Done!