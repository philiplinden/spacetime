"""comms

This module defines components related to agents talking to each other.
"""

import networkx as nx
import mesa
from mesa.model import Model


class Seeker(mesa.Agent):
    """ Finds Runners and tries to tag them.

    Behaviors:
      - When a Seeker is adjacent to a Runner, it changes the other agent to a
        Seeker and itself becomes a Runner.
      - When an agent becomes a Seeker, it must remain in the same grid square
        for the duration of the cooldown period.
      - Each time step, the Seeker...
        1. checks if the cooldown timer has expired. If it has, continue. Else,
           decrement the cooldown and do nothing.
        2. searches its detection radius for Runners.
        3. calculates the angle toward the highest average density of detected
           runners, or a random direction if none are detected.

    Seeker parameters:
      - Seeker speed (grid spaces per jump)
      - Seeker awareness (detection range, in grid spaces)
      - Cooldown time after becoming the seeker (in time steps)
    """

    def __init__(self, unique_id: int, model: Model):
        super().__init__(unique_id, model)

    def step(self):
        pass


class Runner(mesa.Agent):
    """ Flees from the Seeker.

    Behaviors:
      - When a seeker is adjacent to a Runner, it becomes a Seeker and sets the
        cooldown timer.
      - Each time step, a Runner...
        1. searches its detection radius for the Seeker.
        2. chooses a random direction between 90-270 degrees from the direction 
           of the Seeker, or a random direction if the Seeker is not detected.

    Runner parameters:
      - Runner speed (grid spaces per jump)
      - Runner awareness (detection range, in grid spaces)
    """

    def __init__(self, unique_id: int, model: Model):
        super().__init__(unique_id, model)

    def step(self):
        pass


class MarcoPoloModel(mesa.Model):
    """ Just a little game.

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
    """

    def __init__(self, num_runners: int):
        self.num_runners = num_runners
        mesa.space.HexSingleGrid
        self.schedule = mesa.time.RandomActivation(self)

        s = Seeker(0, self)
        self.schedule.add(s)

        for n in range(num_runners):
            r = Runner(n, self)
            self.schedule.add(r)
