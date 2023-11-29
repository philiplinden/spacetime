"""run

This is an executable script that runs the model (and that's it) as recommended
by Mesa: https://mesa.readthedocs.io/en/stable/best-practices.html

Invoke this script with
    mesa runserver demos/marcopolo
"""
from learning.demos.marcopolo.server import marcopolo_server

marcopolo_server.launch(open_browser=True)
