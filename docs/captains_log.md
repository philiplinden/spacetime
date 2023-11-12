# Captain's Log

## 2023-11-12 - PL
Objective: plot trajectories.

### Summary
- Added simple latitude/longitude visualization to the server.

### Notes
It's hard to judge the nyx propagations of agents without a visualization. So
let's add some. There are two "easy" plots to make: line plots and the plots
built in to Nyx.

Took me a while but I finally made a simple grid portrayal for the satellites.
To get it done, I borrowed continuous space visualizations from the Mesa example
[boid flockers](https://github.com/projectmesa/mesa-examples/tree/10985d44091b9ba1ecebd013d2d2252e2116649b/examples/boid_flockers/boid_flockers).

#### Adding a data collector to the model
The `mesa.DataCollector` class init function lets us define reporters from each
agent using lambda functions, so let's use that to grab some orbit information.

## 2023-11-11 - PL
Objective: define a timekeeper agent and preliminary environment.

### Summary
- Removed example code.
- Built a toy example following the mesa.readthedocs.io tutorials: 
  [sandbox.ipynb](../sandbox.ipynb)
- Decided to keep iterating on Python to learn, but may eventually switch to
  Rust for speed if necessary.
- Used Nyx tests as a boilerplate for invoking the Nyx API
    - Created spacecraft objects
    - Propagated orbits forward in time
- Created a basic tunable Mesa server with monte carlo spacecraft agents

### Notes
The tutorials suggest using an interactive environment to begin, and I agree
that it's for the best. The server is a whole other beast and it's too much at
once to deal with them both for now.

The Python documentation for [nyx](https://nyxspace.com/nyxspace/user_guide/start/)
is very rust-centric. I can probably get by from referencing the Python source.
Looks like [hifitime](https://github.com/nyx-space/hifitime) Python docs are a 
bit more mature, but still mostly focused on the rust crate. Should I pivot to
rust? There are [ABM frameworks for rust](https://github.com/krABMaga/krABMaga)
that are plug-and-play...
- Mesa (Python): [Masad, et. al. (2015)](https://conference.scipy.org/proceedings/scipy2015/pdfs/jacqueline_kazil.pdf)
- krABMaga (Rust): [Antelmi, et. al. (2019)](https://link.springer.com/chapter/10.1007/978-981-15-1078-6_2)

For now I'll stick with Python for ease of use. If I run into performance
bottlenecks, it shouldn't be terribly hard to port the meat of the model to a
rust framework.

In the absence of docs, the next best place to look for boilerplate code to get
me started with Nyx is in the tests. As a simple demo, we'll propagate orbits
as the simulation progresses or something.

## 2023-11-10 - PL
Objective: block out the project structure.

### Summary
- Initialized the project metadata and dependencies with Python Poetry
- Added docs via MkDocs
- Set up a working example from [mesa-examples](https://github.com/projectmesa/mesa-examples/tree/main/examples/virus_on_network)

### Notes
I know it's too early to really have documentation or a package manager but
Poetry and MkDocs are so easy so might as well. Besides, they make the whole
project feel so professional, so there's really no reason not to.

As for [Mesa](https://mesa.readthedocs.io/en/stable/overview.html), the examples
library is great for validating the setup and establishing a boilerplate project
setup. The website also has [Best Practices](https://mesa.readthedocs.io/en/stable/best-practices.html)
but it's basically just "use a readme and these file names".
