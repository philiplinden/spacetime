# Captain's Log

## 2023-11-22 - PL
Objective: work on the marco polo model.

### Summary
- Investigated https://github.com/philiplinden/clocss-abm/issues/2
- Resolved https://github.com/philiplinden/clocss-abm/issues/2

### Notes
I moved `get_neighborhood` debugging investigations to GitHub issue
[#2](https://github.com/philiplinden/clocss-abm/issues/2). I ended up fixing it
by trying to explain what was going wrong. Classic.

## 2023-11-21 - PL
Objective: work on the marco polo model.

### Summary
- Moved the orbits demo and marco polo models to their own subdirectories so
  they can be called independently with `mesa runserver clocss/demos/orbits` or
  `mesa runserver clocss/demos/marcopolo`.
- Gave demos their own breathing room in the docs.
- Incremental progress on the Marco Polo model.

### Notes
Another reorg. The orbits demo and marco polo demo are not really part of the
CLOCSS ABM, they're just Mesa and Nyx demos I'm using to learn. I've broken
them out into their own directory to make that clear. That said, both demos
borrow some code from `clocss` so they're not totally decoupled. Doesn't matter
that much to me.

I'm thinking that there should be one class of agents, and a simple flag or enum
that designates whether the agent is a seeker or runner. Since the agents share
the same base class anyway, the only thing that's different is the movement
direction in response to pings. Seekers move toward runners, runners move away
from the seeker and ignore other runners. I think I should reserve agent classes
for fundamental differences in behavior, like a satellite versus a rover, or
a ground station versus a network server.

Made good progress on Marco Polo today. I realized that if I'm gonna make the
agent's role an attribute, there has to be something to inhibit the seekers or
else this is just Tag. I decided Seekers have half the normal detection range.

I spent like 3 hours trying to figure out how to make the grid space wrap around
on the sides. I think this means `torus=True` but as it stands, all the agents
stick to the edges when hitting the edge of the map.

I tried calculating the next index myself, which worked until I hit the edge
where I got `IndexError`s because I wasn't accounting for the torus adjacency.
But when I use the mesa `get_neighborhood` it doesn't wrap either. Maybe I
should skip the neighborhood thing and go straight to calling `torus_adj`?

I'm done for the night ):

## 2023-11-20 - PL
Objective: implement the simple model I designed 

### Summary
- Reformatted `docs/index.md` a little and changed the summary on the `README`
  to a less technical and more straightforward paragraph. Thanks, ChatGPT.

### Notes
In explaining this project with some folks, I found that the given summaries
were too dense with jargon. I used ChatGPT (3.5 Turbo) to give me a few
iterations of a simpler summary. Turned out okay, I think. I got the best result
when providing the bot the entirety of `docs/index.md` and asking it to
summarize without any new information, and then to keep shortening and re-asking
to use plainer language.

[Marco Polo](https://en.wikipedia.org/wiki/Marco_Polo_(game)). That's what my
constraints described. So much easier to move forward after realizing that.
The twist is that the _seeker_ moves toward other nodes, but the _runners_ run
away.

The rules:
  1. There are n Runners and one Seeker.
  2. Runners flee from the Seeker.
  3. The Seeker tries to occupy a grid space adjacent to a Runner.
  4. The first Runner to be adjacent to the Seeker becomes the new Seeker.
      When this happens, the (new) Seeker must wait a period before moving.

World parameters:
  - The number of Runners, n
  - The size of the grid, x and y
  - Whether the world is toroidal (edges wrap, like Pac-Man)

Runner parameters:
  - Runner speed (grid spaces per jump)
  - Runner awareness (detection range, in grid spaces)

Seeker parameters:
  - Seeker speed (grid spaces per jump)
  - Seeker awareness (detection range, in grid spaces)
  - Cooldown time after becoming the seeker (in time steps)

## 2023-11-19 - PL
Objective: learn some more network analysis techniques.

### Summary
- Followed some networkx tutorials.
- Discovered the igraph package.
- Discovered the Multi-level Mesa package.
- Designed a trivial networks Mesa model to work on for learning.

### Notes
Networkx looks to be so powerful and still so easy to use. I can see that some
network analysis (centrality, betweenness) will be useful in understanding how
important (or not) certain nodes are to the network.
[Reference 1](https://www.youtube.com/watch?v=VetBkjcm9Go),
[Reference 2](https://github.com/jdfoote/Intro-to-Programming-and-Data-Science/blob/summer2020/extra_topics/network_analysis.ipynb)

There is another popular network analysis library called
[igraph](https://igraph.org/python/tutorial/0.9.8/tutorial.html), which might
be better for visualizations. From what I can tell, igraph allows properties to
be assigned to nodes and edges, but is much slower when it comes to mutating
the graph ([source](https://igraph.discourse.group/t/igraph-is-much-slower-than-networkx-when-generating-a-graph/853)).
The general consensus is that networkx is better for large, dynamic networks.

Quick searches of using Networkx with Mesa yielded an interesting discovery:
[Multi-level Mesa](https://github.com/tpike3/multilevel_mesa)
> Multi-level Mesa's views complex systems as adaptive networks and uses a
> network graph structure to allow dynamic management of agent modules (groups)
> and model schedules.
The page doesn't do a great job explaining why I'd want to use this over vanilla
Mesa, but it is apparently designed for the kind of problem I'm already trying
to solve... I should read the [paper](https://arxiv.org/pdf/1904.08315.pdf).

> Instead of specifying specific groups and roles, as connections between agents
> change through the dynamics of the ABMs, new modules (groups) can form or
> dissolve and new behaviors can activate or lay dormant. This approach allows
> for neutral networks to exist within any model where certain behaviors may
> only emerge under specific conditions and are not previously seen. Multi-level
> Mesa goes beyond existing approaches by creating a greater synergy between
> network science and ABMs, the interaction of agents produces a dynamic
> network, which in turn alters the behavior of the agents.
>
> [...]
>
> Policy for Multi-level Mesa is understood to be group behavior which alters
> behavior of the agents in their group. If the agent is not part of the group
> it will behave differently.

Considering this package was authored 3 years ago and never updated since then,
I think the best approach is to use Multi-level Mesa as inspiration but not a
dependency.

I want to make a trivial network model with agents, but to be useful the agents
must do something over time. Without overthinking it, here's the plan:

- Each agent has a chance to move one grid space per step in a random direction.
- Each agent has a radius where it can connect to other nodes within its radius.
- The weight of each connection is proportional to its length.
- An agent's likelihood to move is inversly proportional to the total weight of
  all its connections.

## 2023-11-18 - PL
Objective: introduce the repo to new people.

### Summary
- Exported notebooks to the docs as html.
- Wrote a script and tinkered with the github actions workflow to auto-deploy
  example pages for all notebooks.
- Generated some node graphs.

### Notes
I realized that since Jupyter notebooks exported to HTML are able to be
displayed in the docs by simply pasting the raw HTML into the markdown file, I
added a clever cell to the end of the example notebooks so that the whole
notebook can be shown on the GitHub Pages site.
[Did it work?](examples/monte_carlo.md)

```bash
jupyter nbconvert notebooks/monte_carlo.ipynb --to html
cat monte_carlo.html >> docs/monte_carlo.md
```
The next fun things to do with `networkx` are:
- Follow [some](https://math.libretexts.org/Bookshelves/Scientific_Computing_Simulations_and_Modeling/Book:_Introduction_to_the_Modeling_and_Analysis_of_Complex_Systems_(Sayama)/15:_Basics_of_Networks/15.03:_Constructing_Network_Models_with_NetworkX)
  [tutorials](https://www.youtube.com/watch?v=VetBkjcm9Go) to do simple graph
  theory and network analysis.

- Make a simple Mesa server that lets you control the number of nodes and a
  "range" parameter with sliders. The "range" variable controls how many "jumps"
  can go between the "home" node and other nodes. Color edges that fall in range
  the center node.
- Make a random "minefield" of static nodes, and one "minesweeper" that moves
  across the canvas. Change the value of the "minesweeper" node based on how
  many "mine" nodes are within `x` distance of it. Ideally we'd also draw an
  edge between the minesweeper and the mines it can see.

I chatted with Ashley K about the bigger picture. She suggested:

- make a team page and list what they bring to the table
- what ingredients do we have to bake the pie? what is the equivalent of
  preheating the oven?

## 2023-11-17 - PL
Objective: start the network model.

### Summary
- Added `networkx` as a dependency.

### Notes
I started looking at the Mesa example `virus-on-network` for inspiration and
guidance on making a model of networked agents. It imports the package
[networkx](https://networkx.org/), which looks super helpful for this use case.
The docs request citing the package in works using it, so here it is.

> Aric A. Hagberg, Daniel A. Schult and Pieter J. Swart, “Exploring network
> structure, dynamics, and function using NetworkX”, in Proceedings of the 7th
> Python in Science Conference (SciPy2008), Gäel Varoquaux, Travis Vaught, and
> Jarrod Millman (Eds), (Pasadena, CA USA), pp. 11–15, Aug 2008

It is super easy to generate random graphs and plot them with `networkx`! This
will help a ton.

```python
import networkx as nx
G = nx.fast_gnp_random_graph(10, .2)
nx.draw(G)
```

## 2023-11-16 - PL
Objective: start the network model.

### Summary
- Moved the spatial components to `orbits.py`.
- Added additional documentation packages: `mkdocs-material` and `mkautodocs`.
- Refreshed the look and feel of the docs pages.

### Notes
This model has two main components: the motion model and the network model. I'm
going to try to keep this in mind as I organize (and re-organize) the codebase.

Hm, elements are more tightly coupled than I anticipated. `OrbitsModel` needs a
set of `SpacecraftAgent`s to run. The `SpacecraftAgent`s will be the agents that
perform actions and such. Maybe there's a way to add inheritance to things so
that the agents can inherit from a timekeeping agent and an orbiting agent, and
so on. I'm going to segment it off and develop a timekeeping agent separately,
then combine them in a refactor effort whats all the machinery is up and
running.

In the end I decided to put UI components in `server.py`. This way, all of the
visualizations are together. The various model components will be imported by
`model.py` as a nexus of sorts (or perhaps an interface?) for the server to
reach all the various components that will probably show up later. I guess it is
also a rug to sweep messes under...

One thing that seems to be missing is code docs (not just authored docs). I
found an interesting package called
[mkdocstrings](https://mkdocstrings.github.io/) that supposedly automates this
and integrates with [MkDocs](https://www.mkdocs.org/) and is especially tailored
for the [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/)
theme. When searching for docs here, I also found a nifty little package called
`mkdocs-click` for beautifying [Click](https://click.palletsprojects.com/) CLI
docs.

## 2023-11-15 - PL
Objective: report issues with nyx.

### Summary
- Reported issue in Nyx Python: [#250](https://github.com/nyx-space/nyx/issues/250)
- Got Nyx monte carlo orbits working: [notebooks/monte_carlo.ipynb](../notebooks/monte_carlo.ipynb)
- Re-implemented the monte carlo orbit generation in the Mesa model
- Set the server default values to approximate ISS orbit

### Notes
While trying to show how the monte carlo functions weren't working, I proved
myself wrong (in a good way). As shown by [notebooks/monte_carlo.ipynb](../notebooks/monte_carlo.ipynb)
The distributions in SMA and Inclination both work. I didn't have as much luck
with eccentricity, but other parameters worked just fine. I re-integrated the
monte carlo orbit generation into the agent generation code. It's much quicker.

## 2023-11-13 - PL
Objective: clean up the repo.

### Summary
- Organization tweaks
- Refactored the "monte carlo" distributions of orbits because the sliders
  didn't work

### Notes
The code got really messy as I was tinkering and trying to get the visualization
and Nyx to work. I should clean up the code before things get out of hand.

On the topic of rendering plotly figures from the Mesa server, perhaps I can
make use of `plotly.io.to_json` and `plotly.io.from_json` to pass on the figure
data to a canvas. For example, create a simple canvas class that accepts the
figure data as a JSON and a subclass of `VisualizationElement` that renders
the plotly figure from json data.

I discovered that the distribution tweakables for changing agent orbits was
completely broken. For some reason, only SMA was changing when adjusting input
parameters to the Nyx monte carlo functions. Changing inclination and
eccentricity didn't actually have an effect. Since this is just meant to
introduce some variation between the agents' orbits, I abandoned the Nyx
built-ins and used simple gaussian distributions to vary the orbital elements
as each agent is created. The sliders now influence the width of these
distribution functions.

Similarly, changing the mean inclination, raan, or argument of periapsis had
no effect on the orbits that were created by the Nyx monte carlo functions.
Turns out this is because the monte carlo generator initializes with a default
[orbit](https://github.com/nyx-space/nyx/blob/master/src/mc/generator.rs#L344)
and I couldn't figure out how to modify this parameter. When I supplied a custom
orbit to [`generate_orbits()`](https://github.com/nyx-space/nyx/blob/master/src/python/monte_carlo/mod.rs#L48C4-L77)
as an argument, it seemed to be ignored. Rather than debug Nyx, I abandoned this
function.

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

Now that I know I can model trajectories, I'm going to scale back and focus on
the agent interactions. After a working demo of that, I'll merge the two demos
and we have our first real satellite sim.

#### Adding a data collector to the model
The `mesa.DataCollector` class init function lets us define reporters from each
agent using lambda functions, so let's use that to grab some orbit information.

## 2023-11-11 - PL
Objective: define a timekeeper agent and preliminary environment.

### Summary
- Removed example code.
- Built a toy example following the mesa.readthedocs.io tutorials:
  [sandbox.ipynb](../notebooks/sandbox.ipynb)
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
