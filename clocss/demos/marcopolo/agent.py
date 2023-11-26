from collections import namedtuple
from enum import Enum
from math import pi
import random
from typing import List

import mesa


# A radial vector centered at the detector.
#   direction: int - radians from the agent's facing direction.
#   magnitude: float - reciprocal of the Euclidean distance to the detection.
#   role: Role - Is the detected agent a seeker or runner?
DetectionPing = namedtuple("DetectionPing", ["direction", "magnitude", "role"])


class Role(Enum):
    SEEKER = 0
    RUNNER = 1

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
        closest = angle // (pi/4)
        return Direction(closest)

    def to_radians(self) -> float:
        return self.value * (pi/4)


class MarcoPoloAgent(mesa.Agent):
    """Base class for Runners and the Seeker.

    Behaviors
    ---------
      - Search a radius for other agents.
      - Compute the radial density of agents by role.
      - Compute the radial direction toward the maximum density each agent role.
      - Move in a straight line to a grid space given distance away.

    Attributes
    ----------
      - Speed (grid spaces per jump)
      - Detection range (grid spaces, Euclidean distance)
      - Cooldown time after becoming the seeker (in time steps)

    The Seeker:
      The Seeker finds Runners and tries to tag them.

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

    Runners:
      Runners flee from the Seeker.

      - When a seeker is adjacent to a Runner, it becomes a Seeker and sets the
        cooldown timer.
      - Each time step, a Runner...

        1. searches its detection radius for the Seeker.
        2. chooses a random direction between 90-270 degrees from the direction
           of the Seeker, or a random direction if the Seeker is not detected.
    """
    # int: timesteps between movements of 1 gridspace. Must be > 0
    move_interval: int
    # Direction: gridspace direction that the agent considers "forward"
    _facing: Direction

    def __init__(self,
                 unique_id: int,
                 model: mesa.Model,
                 role: Role,
                 speed: int = 1,
                 detection_range: int = 100,
                 facing: float = 0.0,
                 cooldown_timer: int = 0,
    ):
        super().__init__(unique_id, model)
        self.pos = (model.grid.width / 2, model.grid.height / 2)
        self.role = role
        self.move_interval = int(100/max(speed, 1))
        self.search_radius = detection_range
        self.facing = facing
        self.cooldown_timer = cooldown_timer
        self.time_since_last_move = 0

    @property
    def detection_threshold(self) -> float:
        """minimum ping magnitude that results in a detection"""
        r = self.search_radius
        return 2/r if self.role == Role.SEEKER else 1/r

    @property
    def facing(self) -> Direction:
        """gridspace direction that the agent considers "forward"
        """
        return self._facing

    @facing.setter
    def facing(self, angle_from_up: float):
        """direction from gridspace "up" that the agent considers forward

        The gridspace is assumed to be a q-odd hex grid. If the input angle
        is due east or due west, randomly choose an adjacent direction.

        Args:
            angle_from_up (float): radians (CW) from gridspace "up"
        """
        new_facing = Direction.from_radians(angle_from_up)
        if new_facing == Direction.EAST or new_facing == Direction.WEST:
            # randomly pick the cell above or below the edge
            new_facing = Direction(new_facing.value + random.choice([-1, 1]))
        self._facing = new_facing

    @property
    def is_mobile(self):
        if self.cooldown_timer > 0:
            # if there's time banked in the cooldown, you can't move
            self.cooldown_timer -= 1

        elif self.time_since_last_move >= self.move_interval:
            # time to move!
            self.time_since_last_move = 0
            return True

        # if you got here, stay put
        self.time_since_last_move += 1
        return False

    def detect(self) -> List[DetectionPing]:
        """Sense the surrounding space and return a list of discovered agents.

        Seekers have impaired detection capability. If the agent's current role
        is a Seeker, it's detection threshold is doubled.
        Returns:
            List[DetectionPing]: List of pings that were detected
        """
        return []

    def turn(self, detection_pings: List[DetectionPing]) -> float:
        """Choose a direction to move in.

        Seekers want to face toward the maximum density of Runners.
        Runners want to face pi radians from the Seeker's relative position.
        """
        if not detection_pings:
            # pick a random direction
            heading_radians = random.random() * (2*pi)
        else:
            # TODO: pick the heading based on detections
            heading_radians = -1
        return heading_radians

    def _cell_in_front(self):
        """Get the grid coordinate of the cell adjacing to the cell in the
        direction it is facing.

        For a HexGrid the parity of the x coordinate of the point is
        important, the neighborhood can be sketched as:

            Always: (0,-), (0,+)
            When x is even: (-,+), (-,0), (+,+), (+,0)
            When x is odd:  (-,0), (-,-), (+,0), (+,-)

        ```text
                           __N__
                          /     \\
                    _____/  x,y-1\\_____
                NW /     \\       /     \\ NE
                  /x-1,y-1\\_____/x+1,y-1\\
                  \\       /     \\       /
                   \\_____/  x,y  \\_____/
                   /     \\       /     \\
                  /x-1,y  \\_____/x+1,y  \\
                  \\       /     \\       /
                SW \\_____/  x,y+1\\_____/ SE
                         \\       /
                          \\_____/
                              S
        ```
        """
        x, y = self.pos
        if x is None or y is None:
            raise Exception(f'Position must not be None (got {self.pos=})')

        match self.facing:
            case Direction.NORTH:
                new_pos = (x, y-1)
            case Direction.NORTHEAST:
                new_pos = (x+1, y-1)
            case Direction.SOUTHEAST:
                new_pos = (x+1, y)
            case Direction.SOUTH:
                new_pos = (x, y+1)
            case Direction.SOUTHWEST:
                new_pos = (x-1, y)
            case Direction.NORTHWEST:
                new_pos = (x-1, y-1)
            case _:
                raise RuntimeError(f'{self.facing=} is not a valid direction')

        return self.model.grid.torus_adj_2d(new_pos)

    def move(self):
        """Move forward in a straight line.
        """
        target_cell = self._cell_in_front()
        print(f'AGENT {self.unique_id} WANTS TO MOVE TO {target_cell}')
        try:
            self.model.grid.move_agent(self, target_cell)
        except Exception as e:
            print(f'AGENT {self.unique_id} FAILED TO MOVE: {e}')
        # otherwise, can't move!

    def step(self):
        pings = self.detect()
        self.turn(pings)
        if self.is_mobile:
            self.move()
        print(f'AGENT {self.unique_id} MOVED TO {self.pos}')


def portray_agent(agent: MarcoPoloAgent) -> dict:
    return {
        "Shape": "circle",
        "r": .9,
        "Filled": "true",
        "Layer": 0,
        "x": agent.pos[0],
        "y": agent.pos[1],
        "Color": 'red' if agent.role == Role.SEEKER else 'green',
    }
