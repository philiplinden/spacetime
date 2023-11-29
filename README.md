# clocss-abm
an Agent-Based Model of Heterogeneous Lunar Networks for the Cislunar Open Clock
Synchronization System (CLOCSS)

`clocss-abm` employs Agent-Based Modeling to examine lunar networks in the
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

**Documentation is available at [philiplinden.github.io/clocss-abm](https://philiplinden.github.io/clocss-abm/).**

## Getting Started

### Setup (Python)

Install dependencies to a virtual environment using Python Poetry.
```shell
# install python poetry
pip install poetry

# install all dependencies
poetry install
```

### Usage (Python)
There are two demo models that run interactively in the browser.

#### Orbits
The Orbits model uses Nyx to spawn a collection of satellites and propagate their
trajectories. Mesa is used to set up and execute the simulation.

```shell
poetry run mesa runserver learrning/demos/orbits
```

#### Marco Polo
The Marco Polo model uses Mesa to spawn a collection of agents to play the
children's game _Marco Polo_.

```shell
poetry run mesa runserver learning/demos/marcopolo
```
