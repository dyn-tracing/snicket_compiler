# Dependencies Necessary

The dependencies can be found in `./install_deps.sh`.
Scripts for minikube are in `fault_testing.py`.

To build the patched wasme
cd wasm/tools/wasme/cli
make _output/wasme
cp _output/wasme ..
cp _output/wasme ../../../../bin/

