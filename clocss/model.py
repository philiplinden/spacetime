"""model

This module contains the highest level structures that define the model, as 
recommended by Mesa: https://mesa.readthedocs.io/en/stable/best-practices.html
"""
import logging

import mesa
from nyx_space.cosmic import Cosm, Spacecraft, Orbit, SrpConfig
from nyx_space.mission_design import SpacecraftDynamics, propagate
from nyx_space.monte_carlo import StateParameter, generate_orbits
from nyx_space.time import Duration, Epoch, Unit


log = logging.getLogger()


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
        log.debug(f"Initialized agent {self.unique_id}")

    def step(self):
        """what happens when the sim ticks forward"""
        self.spacecraft, self.traj = propagate(self.spacecraft, self.dyn, self.delta_t)
        log.info(f"id: {self.unique_id} epoch: {self.spacecraft.epoch}")


class ClocssModel(mesa.Model):
    """A model with some number of agents"""

    def __init__(
        self,
        num_actors: int,
        epoch: str,
        delta_t: float = 60.0,
        altitude_km: float = 400,
        ecc: float = 1e-4,
        inc_deg: float = 30.5,
        raan_deg: float = 35.0,
        aop_deg: float = 65.0,
        ta_deg: float = 590,
        monte_sma: float = 0.05,
        monte_ecc: float = 0.10,
        monte_inc: float = 0.10,
        monte_kind: str = 'prct',
    ):
        
        # create a bunch of orbits
        cosm = Cosm.de438()
        eme2k = cosm.frame("EME2000")

        e = Epoch(epoch)

        prototype_orbit = Orbit.from_keplerian_altitude(
            altitude_km,
            ecc,
            inc_deg,
            raan_deg,
            aop_deg,
            ta_deg,
            epoch=e,
            frame=eme2k,
        )

        orbits = generate_orbits(
            prototype_orbit,
            [
                (StateParameter("SMA"), monte_sma),
                (StateParameter.Eccentricity, monte_ecc),
                (StateParameter.Inclination, monte_inc),
            ],
            num_actors,
            kind=monte_kind,
        )

        dynamics = SpacecraftDynamics.load("data/dynamics.yaml")
        # create a list of spacecraft agents
        satellites = []
        srp = SrpConfig(2.0)
        for orbit in orbits:
            satellites.append(Spacecraft(orbit, 150.0, 15.0, srp))

        # Create a scheduler
        self.schedule = mesa.time.RandomActivation(self)

        # Create agents
        for i, sc in enumerate(satellites):
            a = SpacecraftAgent(self, i, sc, Unit.Second * float(delta_t),
                                dynamics)
            self.schedule.add(a)
        self.space = mesa.space.MultiGrid(0, 0, True)

    def step(self):
        """Advance the model by one step."""
        log.warn("Step!")
        # The model's step will go here for now this will call the step method
        # of each agent and print the agent's unique_id
        self.schedule.step()
