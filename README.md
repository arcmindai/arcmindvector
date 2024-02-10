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

## Setting up Github Action CI / CD

Get the string using commands below then put it into Github Secrets.
Note: Replace default by the identity name you need.

### DFX_IDENTITY

```
awk 'NF {sub(/\r/, ""); printf "%s\\r\\n",$0;}' ~/.config/dfx/identity/default/identity.pem
```

### DFX_WALLETS

```
cat ~/.config/dfx/identity/default/wallets.json
```

## License

See the [License](LICENSE) file for license rights and limitations (MIT).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details about how to contribute to this project.

## Authors

Code & Architecture: Henry Chan, [henry@arcmindai.app](mailto:henry@arcmindai.app), Twitter: [@kinwo](https://twitter.com/kinwo)

## References

- [Internet Computer](https://internetcomputer.org)
- [Motoko](https://internetcomputer.org/docs/current/motoko/main/motoko)
- [NextJS IC Starter](https://github.com/dappblock/nextjs-ic-starter/)
- [Vessel Package Manager](https://github.com/dfinity/vessel)
