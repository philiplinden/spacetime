"""orbits

Components in this module define spatial elements 
"""
import logging
from pprint import pformat

import mesa
from nyx_space.cosmic import Cosm, Spacecraft, Orbit
from nyx_space.mission_design import (
    SpacecraftDynamics, StateParameter, propagate,
)
from nyx_space.monte_carlo import generate_orbits
from nyx_space.time import Duration, Epoch, Unit


log = logging.getLogger()


class OrbitsModel(mesa.Model):
    """A model with some number of agents"""

    def __init__(
        self,
        num_actors: int,
        epoch: str,
        delta_t: float = 60.0,
        sma: float = 500,
        ecc: float = 1e-4,
        inc_deg: float = 30.5,
        raan_deg: float = 35.0,
        aop_deg: float = 65.0,
        ta_deg: float = 590,
        monte_sma: float = 5,
        monte_raan: float = 10,
    ):
        if num_actors < 1:
            raise ValueError('Must have at least one agent')

        # Create a scheduler and simulation space
        self.schedule = mesa.time.RandomActivation(self)
        self.space = mesa.space.ContinuousSpace(
            x_min=0, x_max=360, y_min=-90, y_max=90, torus=True)

        # create the generic orbit and spacecraft
        cosm = Cosm.de438()
        eme2k = cosm.frame("EME2000")
        dynamics = SpacecraftDynamics.load("clocss/data/dynamics.yaml")
        e = Epoch(epoch)
        template_orbit = Orbit.from_keplerian_altitude(
            sma_altitude_km=sma,
            ecc=ecc,
            inc_deg=inc_deg,
            raan_deg=raan_deg,
            aop_deg=aop_deg,
            ta_deg=ta_deg,
            epoch=e,
            frame=eme2k,
        )
        dry_mass_kg = 100
        
        orbits = generate_orbits(
            template_orbit,
            [
                (StateParameter.SMA, monte_sma),
                (StateParameter.RAAN, monte_raan),
            ],
            num_actors,
            kind='abs',
        )
        # Create a list of spacecraft agents
        for uid, orbit in enumerate(orbits):
            sc = Spacecraft(orbit, dry_mass_kg, 0)
            a = SpacecraftAgent(self, uid, sc, Unit.Second * float(delta_t),
                                dynamics)
            self.space.place_agent(a, (a.longitude, a.latitude))
            self.schedule.add(a)

    def step(self):
        """Advance the model by one step."""
        log.warn("Step!")
        # The model's step will go here for now this will call the step method
        # of each agent and print the agent's unique_id
        self.schedule.step()


class SpacecraftAgent(mesa.Agent):
    """An agent that keeps time"""

    def __init__(
        self,
        model: mesa.Model,
        unique_id: int,
        spacecraft: Spacecraft,
        delta_t: Duration,
        dynamics: SpacecraftDynamics,
    ):
        # pass the parameters to the parent class
        super().__init__(unique_id, model)
        self.spacecraft = spacecraft
        self.delta_t = delta_t
        self.dyn = dynamics
        self.traj = None
        self.update()
        log.debug(f"Initialized agent {self.unique_id}")

    def update(self):
        self.epoch = self.spacecraft.epoch
        self.longitude = self.spacecraft.orbit.geodetic_longitude_deg()
        self.latitude = self.spacecraft.orbit.geodetic_latitude_deg()
        self.x = self.spacecraft.orbit.geodetic_longitude_deg()
        self.y = self.spacecraft.orbit.geodetic_latitude_deg()
        log.info(pformat({'id': self.unique_id,
            'latlon': (self.latitude, self.longitude),
            'orbit': str(self.spacecraft.orbit),
        }))

    def move(self):
        self.model.space.move_agent(self, (self.x, self.y))

    def step(self):
        """what happens when the sim ticks forward"""
        self.spacecraft, self.traj = propagate(self.spacecraft, self.dyn, self.delta_t)
        self.update()
        self.move()
