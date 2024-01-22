The code structure of the model will have a significant impact on its complexity
and performance. It is important for complexity to be minimized and performance
to be maximized in order for the results to be trusted and reproducible.

# rust, not python
I am an intermediate-advanced Python developer. I have absolute faith in my ability to build something at least
marginally functional to meet the goals I set out to achieve with this project. In fact, I already used python in this
project to illustrate core concepts and practice thinking through the problem.

But I chose Rust for this project, not Python.

The primary motivator here is personal growth. I have been looking for a reason to learn Rust for years. This project
affords me low stakes (consequence of failure is ruined pride and that's it), a flexible deadline (no dependents), and a
use case that potentially demands a performant engine to drive it. The conditions are right for me to push outside my
comfort zone and dive head first into the deep end of Rust development.

So far it has certainly been slower (and more tedious) to create functional demos in Rust compared to Python. Part of
that is definitely from me climbing the (steep af) learning curve, and part is inherent to the tenets of Rust that give
it all those superpowers.

I have dipped into Python to "think out loud" and it has been fruitful. My familiarity and comfort working with Rust
projects has dramatically improved in the last month, and I quite enjoy it.

I think it was a wise choice in the long run to go with Rust over python for this project.

# entity-component-system (ecs)
An ECS architecture allows for a modular and scalable design, making it easier
to manage the simulation's complexity. Components store data, and systems
operate on that data, while resources hold shared data needed across systems.
This separation of concerns and data-driven approach is a key characteristic of
ECS. It is a natural fit for a Rust project.

![Unity ECS diagram](https://docs.unity3d.com/Packages/com.unity.entities@0.1/manual/images/ECSBlockDiagram.png)

[Rust Example](https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs)

In our simulation, the _schedule_ is critical. The engine or framework we choose
will need to decide the order of operations each "tick" of the simulation, and
we must consider this order when designing the system.

# choosing a framework
## ecs & abm
ECS frameworks like [Bevy](https://bevyengine.org/) enable real-time,
interactive simulations, but it can be less deterministic and structured than a
discrete-event simulation, since it is [not specially tailored for that purpose](https://github.com/bevyengine/bevy/discussions/1678).

[krABMaga](https://krabmaga.github.io/)[^1] is a purpose-built discrete-events
simulation engine writting in Rust and designed to be a batteries-included tool
for building Agent-Based Models (ABMs). While krABMaga can leverage Bevy to
visualize the model, under the hood it maintains a stricter state and execution
schedule based on the popular [MASON library](https://cs.gmu.edu/~eclab/projects/mason/).

![ECS in krABMaga](https://krabmaga.github.io/images/krabmaga-arch.jpg)

In the end, both are ECS frameworks that can be used here. krABMaga adds some
creature comforts but encourages a certain style. Bevy is the toolbox that can
be used to build whatever you want, but elbow grease is a necessity. Since both
are ECS frameworks, our model can be built from first principles and we can
decide how to run and visualize it later. We might find that the best approach
is using both together.

[^1]: ```bibtex
@ARTICLE{AntelmiASIASIM2019,
  author={Antelmi, A. and Cordasco, G. and D’Auria, M. and De Vinco, D. and Negro, A. and Spagnuolo, C.},
  title={On Evaluating Rust as a Programming Language for the Future of Massive Agent-Based Simulations},
  journal={Communications in Computer and Information Science},
  note={Conference of 19th Asia Simulation Conference, AsiaSim 2019 ; Conference Date: 30 October 2019 Through 1 November 2019;  Conference Code:233729},
  year={2019},
  volume={1094},
  pages={15-28},
  doi={10.1007/978-981-15-1078-6_2},
  issn={18650929},
  isbn={9789811510779},
}```

## astrodynamics
- **[nyx_space](https://nyxspace.com/)**. The documentation is... sparse. Expect to spend a lot of time in the source
  code or DMing the author. There is a wide set of tools tailored specifically for LEO missions up to lunar
  trajectories, including thrust maneuvers and simulated orbit determination from terrestrial ground stations.
- **[particular](https://particular.rs/)**. Easy to get the demos running and the demos are beautiful. I found it
  surprisingly difficult to adapt the demos to my own use cases, despite the apparently simple interface. The demos are
  top tier examples of Bevy and egui. The simulation results "look right" but I was not able to set up a physically
  representative system to test accuracy.
