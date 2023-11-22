"""server

This module contains visualizations and the server itself, as recommended by
Mesa: https://mesa.readthedocs.io/en/stable/best-practices.html
"""
import logging

from mesa.visualization import ModularServer, Slider, Checkbox, CanvasHexGrid


from clocss.demos import marcopolo as mp


log = logging.getLogger()


width, height = 50, 50
marcopolo_params = {
    "num_agents": Slider(
        name="Number of agents",
        value=10,
        min_value=1,
        max_value=100,
        step=1,
    ),
    "torus": Checkbox("world is toroidal", True),
    "speed_mean": Slider(
        name="agent move speed (%)",
        value=50,
        min_value=1,
        max_value=100,
        step=1,
    ),
    "speed_std": Slider(
        name="agent speed standard deviation",
        value=1,
        min_value=0,
        max_value=10,
        step=1,
    ),
    
    "detection_range_mean": Slider(
        name="agent detection range (grid spaces)",
        value=300,
        min_value=50,
        max_value=500,
        step=50,
    ),
    "detection_range_std": Slider(
        name="agent detection range standard deviation",
        value=1,
        min_value=0,
        max_value=10,
        step=1,
    ),
    "tag_cooldown": Slider(
        name="cooldown after getting tagged (ticks)",
        value=3,
        min_value=0,
        max_value=10,
        step=1,
    ),
    "grid_width": width,
    "grid_height": height,
}

canvas = CanvasHexGrid(mp.agent.portray_agent, width, height, 1000, 1000)
marcopolo_server = ModularServer(
    mp.model.MarcoPoloModel, [canvas], model_params=marcopolo_params, port=8521,
)
