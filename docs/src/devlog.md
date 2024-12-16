# Devlog

!!! note

    This page is a log of development notes and thoughts as I work on the
    project. It is not a part of the main body of the book.

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

## 2024-12-15

Here are two videos that really helped me find intuitions for the math of
general relativity:

- https://youtu.be/OpOER8Eec2A?si=22VBvAKu8VV6ye5E
- https://youtu.be/S78h8zQwQe0?si=1ptIvQqqiFlSOqdG

I think it would be helpful to make this app demonstrate these intuitions. Let's
follow the videos' trains of thought and see if we can implement them along the
way. One thing these videos do very well is to build up ideas bit by bit.
Similarly, I think we can build up the app's features bit by bit.

While the math _could_ be implemented with real physical constants, it would be
much easier to see what's happening if we use simple scale factors, especially
if we can change them on the fly.
