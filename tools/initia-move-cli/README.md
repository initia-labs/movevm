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
sudo mv initia-move-cli /usr/local/bin/
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
sudo mv initia-move-cli /usr/local/bin/
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

### How to use it

```bash
Initia Move CLI

Usage: initia-move [OPTIONS] <COMMAND>

Commands:
  build     Build the package at `path`. If no path is provided defaults to current directory
  coverage  Inspect test coverage for this package
  new       Create a new Move package
  test      Run Move unit tests in this package
  help      Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PACKAGE_PATH>                  Path to a package which the command should be run with respect to
  -v                                         Print additional diagnostics if available
  -d, --dev                                  Compile in 'dev' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used if this flag is set. This flag is
                                             useful for development of packages that expose named addresses that are not set to a specific value
      --test                                 Compile in 'test' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used along with any code in the 'tests'
                                             directory
      --override-std <OVERRIDE_STD>          Whether to override the standard library with the given version [possible values: mainnet, testnet, devnet]
      --doc                                  Generate documentation for packages
      --abi                                  Generate ABIs for packages
      --install-dir <INSTALL_DIR>            Installation directory for compiled artifacts. Defaults to current directory
      --force                                Force recompilation of all packages
      --fetch-deps-only                      Only fetch dependency repos to MOVE_HOME
      --skip-fetch-latest-git-deps           Skip fetching latest git dependencies
      --bytecode-version <BYTECODE_VERSION>  Bytecode version to compile move code
      --skip-attribute-checks                Do not complain about an unknown attribute in Move code
      --compiler-version <COMPILER_VERSION>  Compiler version to use
      --language-version <LANGUAGE_VERSION>  Language version to support
      --experiments <EXPERIMENTS>            Experiments for v2 compiler to set to true

```
