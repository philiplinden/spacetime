"""server

This module contains visualizations and the server itself, as recommended by
Mesa: https://mesa.readthedocs.io/en/stable/best-practices.html
"""
import logging
import mesa
from mesa.visualization.ModularVisualization import (
    VisualizationElement, CHART_JS_FILE
)

from .model import ClocssModel, SpacecraftAgent


log = logging.getLogger()

model_params = {
    "num_actors": mesa.visualization.Slider(
        name="Number of agents:",
        value=50,
        min_value=1,
        max_value=100,
        step=1,
    ),
    "epoch": '2023-11-11',
    "delta_t": mesa.visualization.NumberInput(
        name="delta_t",
        value=60.0
    ),
    "altitude_km": mesa.visualization.NumberInput(
        name="altitude_km",
        value=400,
    ),
    "ecc": mesa.visualization.NumberInput(
        name="ecc",
        value=1e-4,
    ),
    "inc_deg": mesa.visualization.NumberInput(
        name="inc_deg",
        value=30.5,
    ),
    "raan_deg": mesa.visualization.NumberInput(
        name="raan_deg",
        value=35.0,
    ),
    "aop_deg": mesa.visualization.NumberInput(
        name="aop_deg",
        value=65.0,
    ),
    "ta_deg": mesa.visualization.NumberInput(
        name="ta_deg",
        value=590,
    ),
    "monte_sma": mesa.visualization.Slider(
        name="SMA variation (%)",
        value=5,
        min_value=0,
        max_value=100,
        step=1,
    ),
    "monte_ecc": mesa.visualization.Slider(
        name="Eccentricity variation (%)",
        value=10,
        min_value=0,
        max_value=100,
        step=1,
    ),
    "monte_inc": mesa.visualization.Slider(
        name="Inclination variation (%)",
        value=10,
        min_value=0,
        max_value=100,
        step=1,
    ),
}


class SimpleCanvas(VisualizationElement):
    """ from mesa examples
    examples/boid_flockers/boid_flockers/SimpleContinuousModule.py
    """
    local_includes = ["clocss/simple_continuous_canvas.js"]
    portrayal_method = None
    canvas_height = 500
    canvas_width = 500

    def __init__(self, portrayal_method, canvas_height=500, canvas_width=500):
        """
        Instantiate a new SimpleCanvas
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
        for obj in model.schedule.agents:
            portrayal = self.portrayal_method(obj)
            x, y = obj.pos
            x = (x - model.space.x_min) / (model.space.x_max - model.space.x_min)
            y = (y - model.space.y_min) / (model.space.y_max - model.space.y_min)
            portrayal["x"] = x
            portrayal["y"] = y
            space_state.append(portrayal)
        return space_state


def satellite_portrayal(agent):
    if agent is None:
        return

    portrayal = {}

    # update portrayal characteristics for each satellite object
    if isinstance(agent, SpacecraftAgent):
        portrayal["Shape"] = "circle"
        portrayal["r"] = 1
        portrayal["Layer"] = 0
        portrayal["Filled"] = "true"
        portrayal["Color"] = '#2596be'
        portrayal["x"] = agent.longitude
        portrayal["y"] = agent.latitude
    else:
        log.error('wrong type!')
    return portrayal


latlon_grid = SimpleCanvas(
    satellite_portrayal, 720, 720)
       

server = mesa.visualization.ModularServer(
    ClocssModel, [latlon_grid], "CLOCSS Model", model_params
)
server.port = 8521
