#!/bin/bash

# Configuration for Ubuntu 18.04
FILE_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd ${FILE_DIR}/..
ENV_DIR="${FILE_DIR}/env"

# basics
sudo apt-get install -y apt-transport-https gnupg2 curl

# fetch the sub modules
git submodule update --init --recursive

# docker
sudo apt install docker.io
# docker without sudo
sudo usermod -aG docker $USER

# kubectl
curl -s https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo apt-key add -
sudo touch /etc/apt/sources.list.d/kubernetes.list
echo "deb http://apt.kubernetes.io/ kubernetes-yakkety main" | sudo tee -a /etc/apt/sources.list.d/kubernetes.list
sudo apt-get update
sudo apt-get install -y kubectl

# minikube
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube_latest_amd64.deb
sudo dpkg -i minikube_latest_amd64.deb
rm minikube_latest_amd64.deb
# configure minikube
# bookinfo requires more memory
minikube config set memory 4096
# need to use docker because we are in a VM
minikube config set driver docker

# bazel
sudo apt install curl gnupg
curl -fsSL https://bazel.build/bazel-release.pub.gpg | gpg --dearmor > bazel.gpg
sudo mv bazel.gpg /etc/apt/trusted.gpg.d/
echo "deb [arch=amd64] https://storage.googleapis.com/bazel-apt stable jdk1.8" | sudo tee /etc/apt/sources.list.d/bazel.list
sudo apt update && sudo apt install bazel

# need prometheus for the API
pip3 install --user prometheus-api-client
# download and unpack istio
cd $ENV_DIR && curl -L https://istio.io/downloadIstio | \
    ISTIO_VERSION=1.8.0 TARGET_ARCH=x86_64 sh - && cd -
# create the bin directory
mkdir -p bin
# download fortio
curl -L https://github.com/fortio/fortio/releases/download/v1.11.4/fortio-linux_x64-1.11.4.tgz \
 | tar --strip-components=2  -C bin usr/bin/fortio -xvzpf -
# build the burst tool
make -C tools/parallel_curl/

# rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
set PATH $HOME/.cargo/bin $PATH
rustup toolchain install nightly

# go
sudo apt install -y golang-go
