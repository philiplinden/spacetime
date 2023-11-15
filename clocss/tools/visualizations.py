"""visualizations

Define visual elements that can be rendered on a Mesa ModularServer.
These elements are generic.
"""
from dataclasses import dataclass, asdict
import logging
from typing import Callable

from mesa.visualization.ModularVisualization import VisualizationElement


log = logging.getLogger()



"""
"x", "y": Coordinates for the cell in which the object is placed.
"Shape": Can be either "circle", "rect", "arrowHead" or a custom image.
    For Circles:
        For Rectangles:
            "w", "h": The width and height of the rectangle, which are in
                        fractions of cell width and height.
            "xAlign", "yAlign": Alignment of the rectangle within the
                                cell. Defaults to 0.5 (center).
        For arrowHead:
            "scale": Proportion scaling as a fraction of cell size.
            "heading_x": represents x direction unit vector.
            "heading_y": represents y direction unit vector.
            For an image:
            The image must be placed in the same directory from which the
            server is launched. An image has the attributes "x", "y",
            "scale", "text" and "text_color".
"""


@dataclass
class PrimitiveShape:
    """
    "x", "y": Coordinates for the cell in which the object is placed.
    "Shape": Can be either "circle", "rect", "arrowHead" or a custom image.
        Note: only "circle" and "rect" are supported by Continuous Canvas
    "Layer": which canvas layer to draw the shape on
    "xAlign", "yAlign": Alignment of the shape within the cell.
                        Defaults to 0.5 (center).
    """
    x: float
    y: float
    Layer: int = 0
    Color: str = "#2596be"
    Filled: str = 'true'
    xAlign: float = 0.5
    yAlign: float = 0.5

    def to_dict(self):
        return asdict(self)


@dataclass
class Circle(PrimitiveShape):
    """
    "r": The radius, defined as a fraction of cell size. r=1 will
            fill the entire cell.
    """
    Shape: str = 'circle'
    r: float = 1


@dataclass
class Rectangle(PrimitiveShape):
    """
    "w", "h": The width and height of the rectangle, which are in
                fractions of cell width and height.
    """
    Shape: str = 'rect'
    w: float = 1
    h: float = 1


class CanvasContinuous(VisualizationElement):
    """Continuous space canvas

    This element is intended to work just like CanvasMultiGrid but for
    continuous spaces.

    adapted from mesa examples
        examples/boid_flockers/boid_flockers/SimpleContinuousModule.py
    """

    local_includes = ["clocss/tools/simple_continuous_canvas.js"]
    portrayal_method = None
    canvas_height = 500
    canvas_width = 500

    def __init__(
        self,
        portrayal_method: Callable,
        canvas_height: int = 500,
        canvas_width: int = 500,
    ):
        """Instantiate a continuous canvas

        A portrayal is a dictionary with the following structure:
        "x", "y": Coordinates for the cell in which the object is placed.
        "Shape": Can be either "circle", "rect", "arrowHead" or a custom image.
            For Circles:
                "r": The radius, defined as a fraction of cell size. r=1 will
                     fill the entire cell.
                "xAlign", "yAlign": Alignment of the circle within the cell.
                                    Defaults to 0.5 (center).
            For Rectangles:
                "w", "h": The width and height of the rectangle, which are in
                          fractions of cell width and height.
                "xAlign", "yAlign": Alignment of the rectangle within the
                                    cell. Defaults to 0.5 (center).
        "Color": The color to draw the shape in; needs to be a valid HTML
                 color, e.g."Red" or "#AA08F8"
        "Filled": either "true" or "false", and determines whether the shape is
                  filled or not.
        "Layer": Layer number of 0 or above; higher-numbered layers are drawn
                 above lower-numbered layers.
        "text": The text to be inscribed inside the Shape. Normally useful for
                showing the unique_id of the agent.
        "text_color": The color to draw the inscribed text. Should be given in
                      conjunction of "text" property.

        Args:
            portrayal_method (Callable): function that takes an agent and
                returns a portrayal dict
            canvas_height (int, optional): Height of canvas area in pixels.
                Defaults to 500.
            canvas_width (int, optional): Width of canvas area in pixels.
                Defaults to 500.
        """
        self.portrayal_method = portrayal_method
        self.canvas_height = canvas_height
        self.canvas_width = canvas_width
        new_element = "new Simple_Continuous_Module({}, {})".format(
            self.canvas_width, self.canvas_height
        )
        self.js_code = "elements.push(" + new_element + ");"

    def render(self, model):
        space_state = []
        if not self.portrayal_method:
            return space_state
        for obj in model.schedule.agents:
            portrayal = self.portrayal_method(obj)
            x, y = obj.pos
            x_min = model.space.x_min
            x_max = model.space.x_max
            y_min = model.space.y_min
            y_max = model.space.y_max
            scaled_x = (x - x_min) / (x_max - x_min)
            scaled_y = (y - y_min) / (y_max - y_min)
            portrayal["x"] = scaled_x
            portrayal["y"] = scaled_y
            space_state.append(portrayal)
        return space_state
