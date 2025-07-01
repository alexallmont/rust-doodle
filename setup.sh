#/usr/bin/env bash
echo Creating environment in .venv...
python3 -m venv .venv
source .venv/bin/activate

echo Patching site-packages in develeper config...
SITE_PACKAGES=$(python -c "import site; print(site.getusersitepackages())")
SED_CMD="s|\".*site-packages|\"$SITE_PACKAGES|g"
sed -i "$SED_CMD" .cargo/config.toml
sed -i "$SED_CMD" .vscode/settings.json

echo Set up remote and local pip dependencies...
pip install --upgrade pip setuptools wheel maturin numpy
maturin develop

echo Done!