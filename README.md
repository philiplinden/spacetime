# spacetime
A Simulation of Heterogeneous Networked Lunar Clocks.

`spacetime` employs Agent-Based Modeling to examine lunar networks in the
Cislunar Open Clock Synchronization System. It focuses on how network topology
and time synchronization evolve in a growing lunar ecosystem, evaluating both
centralized and decentralized Positioning, Navigation, and Timing (PNT) service
network configurations. The Agent-Based Model includes Transmitters, Receivers,
and Peers, simulating their movement, clock drift, and interactions. Key metrics
for assessment encompass accuracy, availability, continuity, interoperability,
cost, and signal integrity. Motivated by advancements in space technologies and
the increase in lunar missions, the project advocates for a resilient
peer-to-peer design that accommodates diverse cislunar ecosystems. The study
aims to predict system performance relative to interconnected assets and
considers interoperability with other missions, including the definition of a
Lunar Reference Frame.

**Documentation is available at [philiplinden.github.io/spacetime](https://philiplinden.github.io/spacetime/).**

## Quickstart

Install the dependencies for your operating system that are needed to run Bevy: [The Bevy Book - Setup](https://bevyengine.org/learn/book/getting-started/setup/)

Then run the project.
```shell
cargo run
```

## Learning
Learning is one of the main objectives of this project. As such, several code demos and iPy notebooks can be found in
the `learning` directory.

The repository contains a mix of Python and Rust. See [the docs](https://philiplinden.github.io/spacetime) for more
discussion on code architectures for this project. It's been an exploration with surprising twists.

There are two Python demo models that run interactively in the browser. Install dependencies to a virtual environment
using [Python Poetry](https://python-poetry.org/). Poetry is then used to spin up the demo projects.

From the `learning` directory:
```shell
# install python poetry
pip install poetry

# install all dependencies
poetry install
```
The **Orbits** model uses Nyx to spawn a collection of satellites and propagate their
trajectories. Mesa is used to set up and execute the simulation.

```shell
poetry run mesa runserver learrning/demos/orbits
```

The **Marco Polo** model uses Mesa to spawn a collection of agents to play the
children's game _Marco Polo_.

```shell
poetry run mesa runserver learning/demos/marcopolo
```

## Attribution

This project borrows from example code provided by
[krABMaga/examples](https://github.com/krABMaga/examples),
[nyx-space/nyx](https://github.com/nyx-space/nyx), and
[Canleskis/particular](https://github.com/Canleskis/particular).

Attributions for all assets are listed in [assets/README.md](assets/README.md).

## License
This repository is distributed under the Mozilla Public License 2.0 (MPL-2.0). MPL-2.0 requires that modifications to
the covered code be released under the same license, thus ensuring improvements remain open-source. However, it allows
the combining of the covered software with proprietary parts, providing flexibility for both academic and commercial
integrations.

For more details, please see the [full text of the license](./LICENSE) or read [a summary by
Github](https://choosealicense.com/licenses/mpl-2.0/).
