"""server

This module contains visualizations and the server itself, as recommended by
Mesa: https://mesa.readthedocs.io/en/stable/best-practices.html
"""
import mesa
from matplotlib.figure import Figure

from .model import ClocssModel


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
    "monte_sma": mesa.visualization.NumberInput(
        name="SMA variation (%)",
        value=0.05,
    ),
    "monte_ecc": mesa.visualization.NumberInput(
        name="Eccentricity variation (%)",
        value=0.10,
    ),
    "monte_inc": mesa.visualization.NumberInput(
        name="Inclination variation (%)",
        value=0.10,
    ),
}


def altitude(model):
    fig = Figure()
    ax = fig.subplots()
    altitudes = []
    for agent in model.schedule.agents:
        orbit = agent.spacecraft.orbit
        altitudes.append(orbit.sma_altitude_km())
    # Note: you have to use Matplotlib's OOP API instead of plt.hist
    # because plt.hist is not thread-safe.
    ax.hist(altitudes, bins=10)
        

server = mesa.visualization.ModularServer(
    ClocssModel, [altitude], "CLOCSS Model", model_params
)
server.port = 8521
