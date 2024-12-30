# Devlog

```admonish warning
This page is a log of development notes and thoughts as I work on the project. It
is not a part of the main body of the book.
```

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

I've said this before and I'll say it again: I am a huge fan of the "retro
radar" aesthetic. It would be fun to lean into that. Here is a Pinterest board
for inspiration: https://pin.it/CgMRWzHFl

## 2024-12-22

I watched a very insightful video series that explains special relativity in a
way that I think is very intuitive. I think it would be helpful to implement
these intuitions here.

- [Special Relativity Intuitions](https://www.youtube.com/playlist?list=PLawLaqps30oBmdbw_D-AI1RQUoCO7Wr1K)
  - [Speed of light is constant in all reference frames](https://youtu.be/hi57CA3GZy4?si=MbhF4UMP-ILTdM9f)
  - [Everything moves at the speed of light](https://youtu.be/TJmgKdc7H34?si=CBHYgsgn1oh3ilZj)
- [General Relativity Intuitions](https://www.youtube.com/playlist?list=PLawLaqps30oAcpVd4r-wj8hGodzpPRYTT)
  - [How gravity bends light even though it has no mass](https://youtu.be/05jFhuRs-w0)
  - [Gravity comes from time curving space](https://youtu.be/OpOER8Eec2A)
  - [Curved spacetime changes trajectories of light and matter](https://www.youtube.com/watch?v=S78h8zQwQe0)

## 2024-12-23

I reworked the docs on relativity, more to come.

I started a new repo to use for manim figures:
[philiplinden/manim-sandbox](https://github.com/philiplinden/manim-sandbox)

I think this repo should basically be an interactive implementation of the
visualizations described
[here](https://youtu.be/wrwgIjBUYVc?si=aY4raU4TI56Kl7XV) and
[here](https://youtu.be/YNqTamaKMC8?si=jMuLrQSQaqbRfsB8)

## 2024-12-30

More relativity docs.
