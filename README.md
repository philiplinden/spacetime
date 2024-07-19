# spacetime
A real(istic) time simulator.

`spacetime` attempts to model time propagation of clocks across the Earth-Moon system. It focuses on how network
topology and time synchronization evolve in a growing lunar ecosystem, evaluating both centralized and decentralized
Positioning, Navigation, and Timing (PNT) service network configurations. 

Motivated by advancements in space technologies and the increase in lunar missions, the project advocates for a
resilient peer-to-peer design that accommodates diverse cislunar ecosystems. The study aims to predict system
performance relative to interconnected assets and considers interoperability with other missions, including the
definition of a [Lunar Coordinate Time (LTC)](https://www.openlunar.org/research/brief-on-lunar-coordinated-time).

**Documentation is available at [philiplinden.github.io/spacetime](https://philiplinden.github.io/spacetime/).**

## Quickstart

Install the dependencies for your operating system that are needed to run Bevy: [The Bevy Book - Setup](https://bevyengine.org/learn/book/getting-started/setup/)

Then run the project.
```shell
cargo run
```

## Learning
Learning is one of the main objectives of this project. See [the docs](https://philiplinden.github.io/spacetime) for more
discussion on code architectures for this project. It's been a true journey of discovery, with surprising twists.

## Attribution
Repository architecture, baseline `Cargo.toml`, and boilerplate modules for Bevy 0.14 were provided by
[TheBevyFlock/bevy_quickstart](https://github.com/TheBevyFlock/bevy_quickstart).

This project also borrows inspiration from 
[krABMaga/examples](https://github.com/krABMaga/examples),
[nyx-space/nyx](https://github.com/nyx-space/nyx), and
[Canleskis/particular](https://github.com/Canleskis/particular).

Attributions for assets are listed in [assets/README.md](assets/README.md).

## License
This repository is distributed under the Mozilla Public License 2.0 (MPL-2.0). MPL-2.0 requires that modifications to
the covered code be released under the same license, thus ensuring improvements remain open-source. However, it allows
the combining of the covered software with proprietary parts, providing flexibility for both academic and commercial
integrations.

For more details, please see the [full text of the license](./LICENSE) or read [a summary by
Github](https://choosealicense.com/licenses/mpl-2.0/).
