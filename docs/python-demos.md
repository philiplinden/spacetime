# Python Explorations
Early on I considered using Python for the entire project. There are some feature-rich Python packages out there and I
am experienced with the language. I implemented a few small demos to feel out how it would be like to do this project
in Python. You can run these demos below.

In the end I decided to use Rust for this project instead. For discussion on that choice, see
[Design Decisions](./design-decisions.md).

## Setup
Install dependencies to a virtual environment using Python Poetry.
```shell
# from /learning

# install python poetry
pip install poetry

# install all dependencies
poetry install
```
## Marco Polo
The Marco Polo model uses Mesa to spawn a collection of agents to play the
children's game _Marco Polo_.

The rules:
    1. There are n Runners and one Seeker.
    2. Runners flee from the Seeker.
    3. The Seeker tries to occupy a grid space adjacent to a Runner.
    4. The first Runner to be adjacent to the Seeker becomes the new Seeker.
        When this happens, the (new) Seeker must wait a period before moving.
    5. No two agents may occupy the same grid space.
    6. The detection threshold of a Seeker is doubled.

```shell
# from /learning
poetry run mesa runserver demos/marcopolo
```

## Orbits
The Orbits model uses Nyx to spawn a collection of satellites and propagate their
trajectories. Mesa is used to set up and execute the simulation.

```shell
# from /learning
poetry run mesa runserver demos/orbits
```
