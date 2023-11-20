"""run

This is an executable script that runs the model (and that's it) as recommended
by Mesa: https://mesa.readthedocs.io/en/stable/best-practices.html

Invoke this script with
    mesa runserver
"""
from clocss.server import orbits_server, networks_server

# orbits_server.launch(open_browser=True)
networks_server.launch(open_browser=True)
