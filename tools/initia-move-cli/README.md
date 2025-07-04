# initia-move-cli

A Light version CLI tool only for the Initia Move compiler.

## Installation

Install _Initia Move CLI_ by downloading the appropriate binary for your architecture using `wget` or `curl`:

### On Macos

### For macOS arm64 (Apple Silicon)

```bash

VERSION=$(curl -s https://api.github.com/repos/initia-labs/movevm/releases/latest | grep '"tag_name":' | cut -d'"' -f4 | cut -c 2-)
curl -L https://github.com/initia-labs/movevm/releases/download/v$VERSION/initia-move-cli-v$VERSION-darwin-arm64.tar.gz -o initia-move-cli-$VERSION-darwin-arm64.tar.gz
tar -xvf initia-move-cli-$VERSION-darwin-arm64.tar.gz

# Install to system path
sudo mkdir -p /usr/local/bin
sudo mv initia-move-cli /usr/local/bin/initia-move
sudo chmod +x /usr/local/bin/initia-move

# Clean up
rm initia-move-cli-$VERSION-darwin-arm64.tar.gz
```

### For macOS x86_64 (amd64)

```bash

VERSION=$(curl -s https://api.github.com/repos/initia-labs/movevm/releases/latest | grep '"tag_name":' | cut -d'"' -f4 | cut -c 2-)
curl -L https://github.com/initia-labs/movevm/releases/download/v$VERSION/initia-move-cli-v$VERSION-darwin-amd64.tar.gz -o initia-move-cli-$VERSION-darwin-amd64.tar.gz
tar -xvf initia-move-cli-$VERSION-darwin-amd64.tar.gz

# Install to system path
sudo mkdir -p /usr/local/bin
sudo mv initia-move-cli /usr/local/bin/initia-move
sudo chmod +x /usr/local/bin/initia-move

# Clean up
rm initia-move-cli-$VERSION-darwin-amd64.tar.gz
```

### On Linux (ubuntu-22.04)

#### **For x86_64 (amd64)**

```bash

VERSION=$(curl -s https://api.github.com/repos/initia-labs/movevm/releases/latest | grep '"tag_name":' | cut -d'"' -f4 | cut -c 2-)
wget https://github.com/initia-labs/movevm/releases/download/v$VERSION/initia-move-cli-v$VERSION-linux-amd64.tar.gz
tar -xvf initia-move-cli-$VERSION-linux-amd64.tar.gz

# Install to system path
sudo install -m 755 initia-move-cli /usr/local/bin/initia-move

# Clean up
rm initia-move-cli-$VERSION-linux-amd64.tar.gz
```

#### **For arm64**

```bash

VERSION=$(curl -s https://api.github.com/repos/initia-labs/movevm/releases/latest | grep '"tag_name":' | cut -d'"' -f4 | cut -c 2-)
wget https://github.com/initia-labs/movevm/releases/download/v$VERSION/initia-move-cli-v$VERSION-linux-arm64.tar.gz
tar -xvf initia-move-cli-$VERSION-linux-arm64.tar.gz

sudo install -m 755 initia-move-cli /usr/local/bin/initia-move

rm initia-move-cli-$VERSION-linux-arm64.tar.gz
```

### Using Docker Container

You can run the Move CLI tool using Docker without installing it locally. Here's how to use it:

```bash
docker run --rm \
  -v "$(pwd):/code:delegated" \
  -w /code \
  ghcr.io/initia-labs/initia-move-cli:latest \
  <command>
```

Example commands:

```bash
# Build Move modules
docker run --rm -v "$(pwd):/code" -w /code ghcr.io/initia-labs/initia-move-cli:latest build

# Run tests
docker run --rm -v "$(pwd):/code" -w /code ghcr.io/initia-labs/initia-move-cli:latest test

# Decode Move module
docker run --rm -v "$(pwd):/code" -w /code ghcr.io/initia-labs/initia-move-cli:latest decode read my_package my_module
```

For easier use, you can create an alias in your shell:

```bash
alias initia-move='docker run --rm -v "$(pwd):/code" -w /code ghcr.io/initia-labs/initia-move-cli:latest'
```

Then use it like the native command:

```bash
initia-move build
initia-move test

### How to use it
initia-move
