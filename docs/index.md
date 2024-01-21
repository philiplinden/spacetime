# spacetime
A Simulation of Heterogeneous Networked Lunar Clocks.

## Abstract
This project aims to use agent-based modeling and simulations to explore how
network topology and time synchronization evolve in a growing lunar ecosystem.
Modeling how the approach may be used in practice and at scale informs decisions
about how to architect a robust cislunar PNT system that scales with the
population.

## Purpose

1. The code in this repository is intended for analytical use for a specific
   problem. It is not meant to be a general-purpose library.
2. This project and its code are evolving as my personal understanding of
   modeling and coding techniques on the way to discovering answers to questions
   about the nature of a possible decentralized PNT service for the Moon.

## Problem statement

The effectiveness of Decentralized PNT versus traditional Lunar GNSS is only meaningful if we can evaluate both paradigms (independently or concurrently) at different scales in population.

| Goals | Apples-to-apples comparisons in performance of a decentralized, centralized, peer-to-peer, transmitter/receiver, and combinations therein based on quantitative metrics. |
| --- | --- |
| Non-goals | This study does not intend to discern optimal constellation architectures or compositions. It also does not claim to produce true-to-life simulations of system performance. |

### The model shall…

1. depend only on open source software.
2. simulate populations of heterogeneous agents.
3. simulate interactions between tens of agents and thousands of agents.
4. simulate events over a time span of days and a span of years.
5. allow an agent’s perception of the environment (epoch, motion) to differ from the “true” environment parameters.
6. simulate spacetime (motion, time with relativity, light propagation) on a cislunar scale.
7. simulate things that inhibit interactions like distance (attenuation, inverse square law) or obstructions (line of sight blocked by a planetary body).
8. allow rulesets for permitted interactions between agents (faction dynamics, transmit/receive limitations, data transfer vs passive sensing, limited communication spectral bands).

### The model should…

1. feed visualizations of populations by type, position, and/or interactions.
2. execute in less than an hour on my laptop.
3. permit rapid execution loops or batched runs with varied model parameters.
4. be easy to replicate by others and on other machines.
