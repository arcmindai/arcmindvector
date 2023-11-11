# Arcmind Vector DB

## Prerequisites

- Install Rust Toolchain using Rustup  
  Follows https://www.rust-lang.org/tools/install
- Install cargo-audit

```
cargo install cargo-audit
```

- Install dfx sdk  
  Follow https://github.com/dfinity/sdk

## Quick Start

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys controller and brain canisters to the local replica
./scripts/provision.sh
```

The provision script will deploy a `controller` canister and a `brain` canister which is owned solely by the `controller`

# Author

Henry Chan henry@controlaltdevelop.com
