from enum import Enum
import logging
from math import pi
import random

from mesa.space import SingleGrid, HexSingleGrid, Coordinate


log = logging.getLogger()


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

    def random_adjacent(self):
        return Direction((self.value + random.choice([-1, 1])) % 8)


class OutOfBoundsException(Exception):
    pass


class SquareGrid(SingleGrid):
    ALLOWED_DIRECTIONS = [
        Direction.NORTH, Direction.EAST, Direction.SOUTH, Direction.WEST
    ]
    
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    def cell_in_front(self, pos: Coordinate, facing: Direction
                      ) -> Coordinate:
        try:
            x, y = pos
        except Exception as e:
            log.warning(f'Error unpacking position {pos}: {e}')
            x = 0
            y = 0

        match facing:
            case Direction.NORTH:
                new_pos = (x, y-1)
            case Direction.EAST:
                new_pos = (x+1, y-1)
            case Direction.WEST:
                new_pos = (x+1, y)
            case Direction.SOUTH:
                new_pos = (x, y+1)
            case _:
                raise RuntimeError(f'{facing=} is not a valid direction')

        # test if the new position coordinates are on the grid
        if self.torus:
            # coordinates off the grid wrap to the other side
            new_pos = self.torus_adj(new_pos)
        else:
            # coordinates off the grid are out of bounds
            if self.out_of_bounds(new_pos):
                raise OutOfBoundsException(
                    f'Coordinate {new_pos} is out of bounds.')
        return new_pos

    def facing(self, angle_from_up: float):
        """direction from gridspace "up" that the agent considers forward

        Args:
            angle_from_up (float): radians (CW) from gridspace "up"
        """
        new_facing = Direction.from_radians(angle_from_up)
        if new_facing not in self.ALLOWED_DIRECTIONS:
            # randomly pick the cell above or below the edge
            new_facing = new_facing.random_adjacent()
        return new_facing


class HexGrid(HexSingleGrid):
    ALLOWED_DIRECTIONS = [
        Direction.NORTH, Direction.NORTHEAST, Direction.NORTHWEST,
        Direction.SOUTH, Direction.SOUTHEAST, Direction.SOUTHWEST,
    ]

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    def cell_in_front(self, pos: Coordinate, facing: Direction
                      ) -> Coordinate:
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
        try:
            x, y = pos
        except Exception as e:
            log.warning(f'Error unpacking position {pos}: {e}')
            x = 0
            y = 0

        match facing:
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
                raise RuntimeError(f'{facing=} is not a valid direction')

        # test if the new position coordinates are on the grid
        if self.torus:
            # coordinates off the grid wrap to the other side
            new_pos = self.torus_adj_2d(new_pos)
        else:
            # coordinates off the grid are out of bounds
            if self.out_of_bounds(new_pos):
                raise OutOfBoundsException(
                    f'Coordinate {new_pos} is out of bounds.')
        return new_pos

    def facing(self, angle_from_up: float):
        """direction from gridspace "up" that the agent considers forward

        The gridspace is assumed to be a q-odd hex grid. If the input angle
        is due east or due west, randomly choose an adjacent direction.

        Args:
            angle_from_up (float): radians (CW) from gridspace "up"
        """
        new_facing = Direction.from_radians(angle_from_up)
        if new_facing not in self.ALLOWED_DIRECTIONS:
            # randomly pick the cell above or below the edge
            new_facing = new_facing.random_adjacent()
        return new_facing
