# Model Structure
The code structure of the model will have a significant impact on its complexity
and performance. It is important for complexity to be minimized and performance
to be maximized in order for the results to be trusted and reproducible.

## Entity-Component-System (ECS)

An ECS architecture allows for a modular and scalable design, making it easier
to manage the simulation's complexity. Components store data, and systems
operate on that data, while resources hold shared data needed across systems.
This separation of concerns and data-driven approach is a key characteristic of
ECS.

![Unity ECS diagram](https://docs.unity3d.com/Packages/com.unity.entities@0.1/manual/images/ECSBlockDiagram.png)

[Rust Example](https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs)

In our simulation, the _schedule_ is critical. The engine or framework we choose
will need to decide the order of operations each "tick" of the simulation, and
we must consider this order when designing the system.

## Choosing a Framework
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
