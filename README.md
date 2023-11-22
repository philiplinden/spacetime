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

# Instructions
```shell
# install python poetry
pip install poetry

# install all dependencies
poetry install

# just start the server!
poetry run mesa runserver clocss/demos/orbits
```
