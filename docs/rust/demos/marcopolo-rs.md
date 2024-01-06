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
cd learning/demos/marcopolo-rs
cargo run
```

# Code Reference

