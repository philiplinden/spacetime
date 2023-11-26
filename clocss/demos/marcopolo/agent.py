from enum import Enum
import logging
from math import pi
import random
from typing import Iterable, List

import mesa
import numpy as np

from clocss.tools import grid


log = logging.getLogger()


class Role(Enum):
    SEEKER = 0
    RUNNER = 1


class DetectionPing:
    ''' A radial vector centered at the detector. '''
    #: float - radians from the agent's facing direction.
    direction: float
    #: float - reciprocal of the Euclidean distance to the detection.
    amplitude: float
    #: Role - Is the detected agent a seeker or runner?
    signature: Role

    def __init__(self, source, other):
        self.signature = other.role

        origin_to_source = np.array(source.pos)
        origin_to_other = np.array(other.pos)
        source_to_other = origin_to_source - origin_to_other
        length = float(np.linalg.norm(source_to_other))

        self.amplitude = 1 / length

        heading = source.facing.to_radians() # 0 == aligned with y-axis
        angle_to_ping = np.arccos(source_to_other[1] / length)
        self.direction = angle_to_ping - heading


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
    _facing: grid.Direction

    def __init__(self,
                 unique_id: int,
                 model: mesa.Model,
                 speed: int = 1,
                 detection_range: int = 100,
                 facing: float = 0.0,
                 cooldown_duration: int = 0,
    ):
        super().__init__(unique_id, model)
        self.pos: grid.Coordinate = (0, 0)
        self.gridspace = model.grid
        self.role = Role.RUNNER
        self.move_interval = int(100/max(speed, 1))
        self.search_radius = detection_range
        self.facing = facing
        self.cooldown_duration = cooldown_duration
        self.cooldown_timer = 0
        self.time_since_last_move = 0

    @property
    def detection_threshold(self) -> float:
        """minimum ping magnitude that results in a detection"""
        r = self.search_radius
        return 2/r if self.role == Role.SEEKER else 1/r

    @property
    def facing(self) -> grid.Direction:
        """gridspace direction that the agent considers "forward"
        """
        return self._facing

    @facing.setter
    def facing(self, angle_from_up: float | grid.Direction):
        if isinstance(angle_from_up, float):
            self._facing = self.gridspace.facing(angle_from_up)
        elif isinstance(angle_from_up, grid.Direction):
            self._facing = angle_from_up
        else:
            raise TypeError(
                f'Must be float or Direction, not {type(angle_from_up)}')

    def _cell_in_front(self):
        return self.gridspace.cell_in_front(self.pos, self.facing)

    @property
    def is_frozen(self):
        if self.cooldown_timer > 0:
            # if there's time banked in the cooldown, you can't move
            self.cooldown_timer -= 1
        elif self.time_since_last_move >= self.move_interval:
            # time to move!
            self.time_since_last_move = 0
            return False
        # if you got here, stay put
        self.time_since_last_move += 1
        return True

    def tag_other(self, other: mesa.Agent):
        log.info(f'Agent {self.unique_id} tagged Agent {other.unique_id}.')
        self.role = Role.RUNNER
        other.tagged()

    def tagged(self):
        self.cooldown_timer += self.cooldown_duration
        self.role = Role.SEEKER
        log.info(f'Agent {self.unique_id} is now the SEEKER!')

    def detect(self) -> List[DetectionPing]:
        """Sense the surrounding space and return a list of discovered agents.

        Seekers have impaired detection capability. If the agent's current role
        is a Seeker, it's detection threshold is doubled.
        Returns:
            List[DetectionPing]: List of pings that were detected
        """
        detections = []
        # FIXME: this is really expensive, O(N^2)!
        for agent in self.model.agents:
            if agent.unique_id == self.unique_id:
                # don't detect yourself
                continue
            maybe_detection = DetectionPing(self, agent)
            # tag anyone if you find them in an adjacent cell
            if self.role == Role.SEEKER and maybe_detection.amplitude >= 1:
                self.tag_other(agent)
                # the only detection that matters now is this one
                maybe_detection.signature = Role.SEEKER
                detections = [maybe_detection]
                break
            # otherwise just sense their presence if close enough
            elif maybe_detection.amplitude > self.detection_threshold:
                detections.append(maybe_detection)
        return detections

    @staticmethod
    def _max_radial_density(radial_directions: Iterable[float],
                            nbins: int = 360):
        """Compute the radial direction toward the highest probability density.

        This function takes a list of radial directions and computes the
        histogram with NBINS equally spaced bins between 0 and 2pi.
        """
        # project directions outside of [0,2pi) onto the unit circle
        angles = [theta % 2*pi for theta in radial_directions]
        density, angle = np.histogram(angles, bins=nbins, range=(0, 2*pi),
                                      density=True)
        return angle[np.argmax(density)]

    def turn(self, detection_pings: List[DetectionPing] = []):
        """Choose a direction to move in.

        Seekers want to face toward the maximum density of Runners.
        Runners want to face pi radians from the Seeker's relative position.
        """
        if not detection_pings:
            # pick a random direction
            heading_radians = random.random() * (2*pi)
        else:
            max_density_heading = self._max_radial_density(
                [ping.direction for ping in detection_pings \
                    if ping.signature != self.role])
            if self.role == Role.RUNNER:
                # run away!
                heading_radians = pi + max_density_heading
            else:
                # go get em!
                heading_radians = max_density_heading
        self.facing = heading_radians

    def move(self):
        """Move forward in a straight line.
        """
        target_cell = self._cell_in_front()
        if self.gridspace.is_cell_empty(target_cell):
            self.gridspace.move_agent(self, target_cell)
        else:
            self.facing = self.facing.random_adjacent()
            log.debug(f'Turning to {self.facing} and trying again...')
            self.move()

    def step(self):
        if not self.is_frozen:
            pings = self.detect()
            self.turn(pings)
            try:
                self.move()
            except Exception as e:
                log.debug(f'Agent {self.unique_id} failed to move ({e})')


def portray_agent_on_hex(agent: MarcoPoloAgent) -> dict:
    return {
        "Shape": "circle",
        "r": 1,
        "text": agent.unique_id,
        "text_color": "white",
        "Filled": "true",
        "Layer": 0,
        "x": agent.pos[0],
        "y": agent.pos[1],
        "Color": 'red' if agent.role == Role.SEEKER else 'green',
    }


def portray_agent_on_square(agent: MarcoPoloAgent) -> dict:
    return {
        "Shape": "arrowHead",
        "scale": 1,
        "heading_x": np.cos(agent.facing.to_radians()),
        "heading_y": np.sin(agent.facing.to_radians()),
        "text": agent.unique_id,
        "text_color": "white",
        "Filled": "true",
        "Layer": 0,
        "x": agent.pos[0],
        "y": agent.pos[1],
        "Color": 'red' if agent.role == Role.SEEKER else 'green',
    }
