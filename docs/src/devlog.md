# Devlog

!!! note

    This page is a log of development notes and thoughts as I work on the project.
    It is not a part of the main body of the book.

## 2024-12-08

I switched the docs to use mdbook today and I'm feeling good about diving back
in to this project. Previous devlogs are available on the repository's
[archive](https://github.com/philiplinden/spacetime/tree/main/archive/devlog).

Since the last devlog I've gotten a lot more experience with Rust and Bevy. I
have a strong sense of how to structure a Bevy project and I'm comfortable with
its features now, so I'm ready to drop krabmaga and start building the project
from scratch.

To start, I'm going to build the simulation without any rendering or graphics.
Just enough to see what's happening through logs or maybe egui. Unlike previous
attempts, I'm going to focus on implementing the math from the research papers I
found, rather than trying to jump straight to the end goal.
