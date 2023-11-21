"""comms

This module defines components related to agents talking to each other.
"""

from abc import abstractmethod
from collections import namedtuple
from enum import Enum
from math import pi
import random
from typing import List

import networkx as nx
import mesa
from mesa.model import Model


# A radial vector centered at the detector.
#   direction: int - radians from the agent's facing direction.
#   magnitude: float - reciprocal of the Euclidean distance to the detection.
DetectionPing = namedtuple("DetectionPing", ["direction", "magnitude"])


class Direction(Enum):
    """ Radial angle from gridspace "up".

    The usefulness of this enum is to snap movement or facing to the gridspace.
    """
    NORTH = 0
    NORTHEAST = 1
    EAST = 2
    SOUTHEAST = 3
    SOUTH = 4
    SOUTHWEST = 5
    WEST = 6
    NORTHWEST = 7

    @staticmethod
    def from_radians(angle_from_up: float):
        angle = angle_from_up % (2 * pi)
        closest = angle % (pi/4)
        return Direction(closest)

    @staticmethod
    def to_radians():
        #TODO
        pass


class MarcoPoloAgent(mesa.Agent):
    """Base class for Runners and the Seeker.

    Shared behaviors each time step:
      - Search a radius for other agents.
      - Compute the radial density of agents by type.
      - Compute the radial direction toward the maximum density of an agent type
      - Move in a straight line to a grid space given distance away.
    """
    # int: number of grid spaces to traverse per step in the facing direction
    speed: int
    # float: minimum ping magnitude that results in a detection
    detection_threshold: float
    # Direction: gridspace direction that the agent considers "up"
    _facing: Direction

    def __init__(self,
                 unique_id: int,
                 model: Model,
                 speed: int,
                 range: int,
                 facing: float,
    ):
        super().__init__(unique_id, model)

    @property
    def facing(self) -> float:
        """ the angle from gridspace "up" that the agent considers forward
        """
        return self._facing.value

    @facing.setter
    def facing(self, angle_from_up: float):
        self._facing = Direction.from_radians(angle_from_up)
    
    @abstractmethod
    def detect(self, kind: mesa.Agent) -> List[DetectionPing]:
        """
        """

    def step(self):
        pass


class Seeker(MarcoPoloAgent):
    """Finds Runners and tries to tag them.

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

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    def step(self):
        pass


class Runner(MarcoPoloAgent):
    """Flees from the Seeker.

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

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    def step(self):
        pass


class MarcoPoloModel(mesa.Model):
    """Just a little game.

    The rules:
      1. There are n Runners and one Seeker.
      2. Runners flee from the Seeker.
      3. The Seeker tries to occupy a grid space adjacent to a Runner.
      4. The first Runner to be adjacent to the Seeker becomes the new Seeker.
         When this happens, the (new) Seeker must wait a period before moving.
      5. No two agents may occupy the same grid space.

    World parameters:
      - The number of Runners, n
      - The size of the grid, x and y
      - Whether the world is toroidal (edges wrap, like Pac-Man)
    """

    def __init__(
        self,
        num_runners: int,
        grid_width=500,
        grid_height=500,
        torus=False,
        runner_speed=1,
        runner_range=10,
        seeker_speed=1,
        seeker_range=100,
        
    ):
        super().__init__()

        self.num_runners = num_runners
        self.grid = mesa.space.HexSingleGrid(grid_width, grid_height, torus=torus)

        self.schedule = mesa.time.RandomActivation(self)
        agents = []

        s = Seeker(0, self)
        self.schedule.add(s)
        agents.append(s)

        for n in range(num_runners):
            r = Runner(n, self, runner_speed, runner_range)
            self.schedule.add(r)
            agents.append(r)

        for a in agents:
            # Add the agent to a random grid cell
            x = random.randrange(self.grid.width)
            y = random.randrange(self.grid.height)
            self.grid.place_agent(a, (x, y))

    def step(self):
        pass
