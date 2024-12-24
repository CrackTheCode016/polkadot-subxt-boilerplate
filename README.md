<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a id="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/CrackTheCode016/polkadot-subxt-boilerplate">
    <img src="https://cryptologos.cc/logos/polkadot-new-dot-logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Subxt Rust Boilerplate</h3>

  <p align="center">
    Quickly get started with experimenting with Subxt, a Rust library for interacting with Polkadot SDK-based blockchains.
    <br />
  </p>
</div>

<!-- GETTING STARTED -->
## Getting Started

Getting started should be simple - simply clone this repository, `cargo build`, and you're off to the races.

### Prerequisites


- Be sure you have [Rust](https://www.rust-lang.org/tools/install) and its associated tooling installed.
- Install the `subxt-cli`:
  

   ```sh
   cargo install subxt-cli
   ```

### Installation

```sh
cargo build # This will install and build everything
```

<!-- USAGE EXAMPLES -->
## Usage

To run, you can simply use `cargo`:

```sh
cargo run
```

## Adding a custom chain

If you have a custom-built chain, such as one built with the "[Zero to Hero" guide for building custom blockchains](https://docs.polkadot.com/tutorials/polkadot-sdk/parachains/zero-to-hero/) using the Polkadot SDK, here's how you can accommodate the boilerplate to use that instead of a production network.

1. Make sure you have `subxt-cli` installed (see: [Prerequisites](#prerequisites)), and run the following for your network (`localhost` or deployed or a url works):
  ```sh
  subxt metadata -f bytes --url ws://localhost:9944 > ./metadata/custom.scale
  ```

2. In your code, change (or add, it is possible to have more than one config / metadata at a time!) the code from AssetHub's metadata to your custom one:

```rust
// Change this:
#[subxt::subxt(runtime_metadata_path = "./metadata/polkadot_relay_chain.scale")]
pub mod polkadot {}

// To this:
#[subxt::subxt(runtime_metadata_path = "./metadata/custom.scale")]
pub mod custom {}

// Also add the config!
type CustomConfig = SubstrateConfig;
```

1. Wherever `polkadot` is used, change it to `custom`, and be sure to change `PolkadotConfig` to `CustomConfig`, i.e.,: 

```rust
let api = OnlineClient::<CustomConfig>::from_url(
    "ws://localhost:9944"
).await?;

// Build a balance transfer extrinsic.
let runtime_upgrade_query = custom::storage().system().last_runtime_upgrade();
```

If you need to create a custom configuration that isn't the "generic" Substrate one, follow this [guide](https://docs.rs/subxt/latest/subxt/book/setup/config/index.html).

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/CrackTheCode016/polkadot-subxt-boilerplate.svg?style=for-the-badge
[contributors-url]: https://github.com/CrackTheCode016/polkadot-subxt-boilerplate/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/CrackTheCode016/polkadot-subxt-boilerplate.svg?style=for-the-badge
[forks-url]: https://github.com/CrackTheCode016/polkadot-subxt-boilerplate/network/members
[stars-shield]: https://img.shields.io/github/stars/CrackTheCode016/polkadot-subxt-boilerplate.svg?style=for-the-badge
[stars-url]: https://github.com/CrackTheCode016/polkadot-subxt-boilerplate/stargazers
[issues-shield]: https://img.shields.io/github/issues/CrackTheCode016/polkadot-subxt-boilerplate.svg?style=for-the-badge
[issues-url]: https://github.com/CrackTheCode016/polkadot-subxt-boilerplate/issues