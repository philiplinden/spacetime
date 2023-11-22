"""server

This module contains visualizations and the server itself, as recommended by
Mesa: https://mesa.readthedocs.io/en/stable/best-practices.html
"""
import logging

from mesa.visualization import ModularServer, NumberInput, Slider

from clocss.tools.visualizations import CanvasContinuous, Circle
from clocss.demos.orbits.model import OrbitsModel, SpacecraftAgent


log = logging.getLogger()
R_EARTH_KM = 6387

orbits_params = {
    "num_actors": Slider(
        name="Number of agents:",
        value=50,
        min_value=1,
        max_value=100,
        step=1,
    ),
    "epoch": "2023-01-01 00:00:00.0000 UTC",
    "delta_t": NumberInput(name="delta_t", value=60.0),
    "sma": NumberInput(
        name="sma",
        value=6738-R_EARTH_KM,
    ),
    "ecc": NumberInput(
        name="ecc",
        value=0.0005177,
    ),
    "inc_deg": NumberInput(
        name="inc_deg",
        value=51.6430,
    ),
    "raan_deg": NumberInput(
        name="raan_deg",
        value=300.8040,
    ),
    "aop_deg": NumberInput(
        name="aop_deg",
        value=301.9962,
    ),
    "ta_deg": NumberInput(
        name="ta_deg",
        value=320.4914,
    ),
    "monte_sma": Slider(
        name="SMA variation (km)",
        value=50,
        min_value=0,
        max_value=100,
        step=1,
    ),
    "monte_raan": Slider(
        name="RAAN variation (deg)",
        value=90,
        min_value=0,
        max_value=360,
        step=1,
    ),
}


def satellite_portrayal(agent: SpacecraftAgent) -> dict:
    """Define a portrayal for a SpacecraftAgent

    Args:
        agent (SpacecraftAgent): _description_

    Returns:
        dict: _description_
    """

    # update portrayal characteristics for each satellite object
    return Circle(
        r = 1,
        Layer = 0,
        Color = "#2596be",
        x = agent.x,
        y = agent.y,
    ).to_dict()


latlon_grid = CanvasContinuous(satellite_portrayal, 720, 720)
orbits_server = ModularServer(
    OrbitsModel, [latlon_grid], "Orbits Model", orbits_params
)
orbits_server.port = 8521
