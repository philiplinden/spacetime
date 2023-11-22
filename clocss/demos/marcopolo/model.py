"""model

This module contains the highest level structures that define the model, as
recommended by Mesa: https://mesa.readthedocs.io/en/stable/best-practices.html
"""
from math import pi
import random

import mesa

from clocss.demos.marcopolo.agent import MarcoPoloAgent, Role


class MarcoPoloModel(mesa.Model):
    """Just a little game.

    The rules:
      1. There are n Runners and one Seeker.
      2. Runners flee from the Seeker.
      3. The Seeker tries to occupy a grid space adjacent to a Runner.
      4. The first Runner to be adjacent to the Seeker becomes the new Seeker.
         When this happens, the (new) Seeker must wait a period before moving.
      5. No two agents may occupy the same grid space.
      6. The detection threshold of a Seeker is doubled.

    The world is an "odd-q" hexagonal grid, indexed from the top right. Screen
    space "up" is gridspace "up" or agent-space "north". Movement occurs in
    whole number translations across shared edges, so translations are not
    permitted directly Eastward or directly Westward.

    ```text
    odd-q hexagonal grid coordinate frame
       ___     ___
      /0,0\\___/2,0\\___
      \\___/1,0\\___/3,0\\
      /0,1\\___/2,1\\___/
      \\___/1,1\\___/3,1\\
      /0,2\\___/2,2\\___/
      \\___/1,2\\___/3,2\\
          \\___/   \\___/

    see also: https://www.redblobgames.com/grids/hexagons/#coordinates
    ```
    """

    def __init__(
        self,
        num_agents: int = 10,
        grid_width: int = 500,
        grid_height: int = 500,
        torus: bool = True,
        speed_mean: float = 5,
        speed_std: float = 1,
        detection_range_mean: float = 100,
        detection_range_std: float = 1,
        tag_cooldown: int = 10,
    ):
        """Create a Marco Polo world.

        Args:
            num_agents (int, optional): Number of agents. Defaults to 10.
            grid_width (int, optional): Width of world space. Defaults to 500.
            grid_height (int, optional): Height of world space. Defaults to 500.
            torus (bool, optional): Whether the edges of the world wrap, like
                                    in Pac-Man. Defaults to False.
            speed_mean (float, optional): Average time steps between movements.
                                          Defaults to 5.
            speed_std (float, optional): Standard deviation speed of the
                                         population. Defaults to 1.
            detection_range_mean (float, optional): Average Euclidian distance
                                                    that agents can detect
                                                    others. Defaults to 100.
            detection_range_std (float, optional): Standard deviation detection
                                                   range of the population.
                                                   Defaults to 1.
            tag_cooldown (int, optional): Minimum time steps an agent spends
                                          immobilized after becoming a Seeker.
                                          Defaults to 10.

        Raises:
            ValueError: Cannot have more agents than grid spaces.
        """
        super().__init__()
        self.grid = mesa.space.HexSingleGrid(grid_width, grid_height,
                                             torus=torus)
        self.schedule = mesa.time.RandomActivation(self)

        # sanity check to avoid an infinite loop while placing agents
        if num_agents > self.grid.num_cells:
            raise ValueError('Too many agents! Only one agent is allowed per '
                             'grid space. Try again with fewer than '
                             f'{self.grid.num_cells} agents')

        for n in range(num_agents):
            if n == 0:
                role = Role.SEEKER
                cooldown_timer = max(tag_cooldown, 0)
            else:
                role = Role.RUNNER
                cooldown_timer = 0

            a = MarcoPoloAgent(
                unique_id=n,
                model=self,
                role=role,
                facing=random.random() * (2*pi),
                speed=int(random.gauss(speed_mean, max(speed_std, 0))),
                detection_range=int(random.gauss(
                    detection_range_mean, max(detection_range_std, 0))),
                cooldown_timer=cooldown_timer,
            )
            self.place_agent(a)
            self.schedule.add(a)

    def place_agent(self, agent: mesa.Agent):
        """Add the agent to a random unoccupied grid cell"""
        x = random.randint(0, self.grid.width)
        y = random.randint(0, self.grid.height)
        try:
            self.grid.place_agent(agent, (x, y))
        except Exception:
            # place it somewhere else (recursive)
            self.place_agent(agent)

    def step(self):
        """Advance the model by one step."""
        self.schedule.step()
