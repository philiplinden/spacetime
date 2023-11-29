# Model Requirements

Owner: Philip Linden
Tags: Design, Engineering

**Table of contents**

# Problem statement

The effectiveness of Decentralized PNT versus traditional Lunar GNSS is only meaningful if we can evaluate both paradigms (independently or concurrently) at different scales in population.

See [clocss-abm](https://www.notion.so/clocss-abm-4b7232aa0f8f4af9926db05ba0e7fa11?pvs=21) 

| Goals | Apples-to-apples comparisons in performance of a decentralized, centralized, peer-to-peer, transmitter/receiver, and combinations therein based on quantitative metrics. |
| --- | --- |
| Non-goals | This study does not intend to discern optimal constellation architectures or compositions. It also does not claim to produce true-to-life simulations of system performance. |

## The model shall…

1. depend only on open source software.
2. simulate populations of heterogeneous agents.
3. simulate interactions between tens of agents and thousands of agents.
4. simulate events over a time span of days and a span of years.
5. allow an agent’s perception of the environment (epoch, motion) to differ from the “true” environment parameters.
6. simulate spacetime (motion, time with relativity, light propagation) on a cislunar scale.
7. simulate things that inhibit interactions like distance (attenuation, inverse square law) or obstructions (line of sight blocked by a planetary body).
8. allow rulesets for permitted interactions between agents (faction dynamics, transmit/receive limitations, data transfer vs passive sensing, limited communication spectral bands).

## The model should…

1. feed visualizations of populations by type, position, and/or interactions.
2. execute in less than an hour on my laptop.
3. permit rapid execution loops or batched runs with varied model parameters.
4. be easy to replicate by others and on other machines.

# Metrics

These metrics are parameters that should be instrumented in the model.

### **Accuracy**

- *Sync precision* - Standard deviation absolute time across the node population
- *Holdover* - Average clock drift from true time before syncing
- *Latency (jitter)* - Round trip time, packet delay variation

### **Availability**

- *Capacity* - % bandwidth used, peak bandwidth used, sqkm of coverage
- *Throughput* - Total available bandwidth, # concurrent links

### **Continuity**

- *Roaming ability* - # available links, time between links
- *Failure & recovery rate* - Mean time between failures, mean time to restore

### **Interoperability**

- *Standards compatibility* - Meets LunaNet and Moonlight requirements
- *Technology compatibility* - # comms spectral bands, clients served per band

### **Cost**

- *Nodes required for service* - # service nodes, # clients per provider node
- *Hardware required* - $ per clock, # clocks, $ invested per client served

### **Signal Integrity**

- *Packet integrity* - packet loss rate, packet delivery ratio, % duplicate packets
- *Channel dominance* - signal-to-noise ratio, jam-to-signal ratio

# Agents

Agents are the generic term for “things with clocks” in this context. It might be a satellite, rover, ground station, etc. This section aims to identify the minimal set of traits that are required for agents in this model. Hopefully this will tease out areas of overlap between different types of agents and a blueprint for the model will take shape.

## Attributes

Parent

Some agents are attached to other entities, like ground stations or rovers. If an agent must translate and rotate together with another entity, that other entity is 

### Mass

Total mass of the agent in kg.

### Clock Precision

Likelihood of a missed or double counted tick of the clock

### Radio Band

An agent may only transmit or receive messages from other agents if the message is within the agent’s radio band.

### Radio Field of View (FoV)

An agent may only transmit or receive messages from other agents along a vector that originates at the agent* and lies within a cone** with a given field of view (FoV) angle. For example, an FoV of $2\pi$ would define an omnidirectional radio’s field of view.

A message is only received if the message vector between the sender’s radio and receiver’s radio is completely encapsulated by both agents’ radio fields of view.

*For simplicity’s sake, let’s assume agents are points with no shape.

**A possible extension of this is to define the radio field of view as a set of lobes instead of a cone, but a cone is the simplest shape that meets this need.

### Radio Transmitter Power

Messages originating at the agent have an initial power (in dBm). The message signal power is distributed across the radio’s entire field of view. Signal power falls off with the inverse square law.

### Radio Receiver Noise Floor

The power (in dBm) of an agent’s receiver’s noise floor. See [Radio Receiver Sensitivity](https://www.notion.so/Radio-Receiver-Sensitivity-d3a17af383964900b953681fb21108a7?pvs=21).

### Radio Receiver Sensitivity

An agent may only detect a message if the instantaneous signal power at the receiver is greater than a certain threshold. The ratio between the signal power and the noise floor power is the signal-to-noise ratio (SNR). Receiver sensitivity is the minimum SNR that results in a message being received, otherwise a message is “dropped” or ignored by the receiver.

## States

### Position, Velocity, and Acceleration

Agents exist in a 2D or 3D simulation space* and may translate across the space. Agents also have a heading and may rotate. Agents have translational and rotational velocity and accelerations. In short, I want some rigid body physics here. The agent may only change its velocity through accelerations.

Accelerations may be imparted on an agent from itself (thrust) or the environment (gravity, collisions).

*Obviously 3D space is the goal, but all fundamental principals can be demonstrated on a single 2D plane.

### Clock Epoch

Agents keep track of time with imperfect clocks. The agent’s observation of the epoch may diverge from the environment’s true epoch.

### Stratum

How far removed from canonical time source.

# Environment

gravity field

true epoch

inert bodies (earth, moon)

# Visualization

agents

position, heading

type

state

field of view

trajectory

environment

agents

inert bodies

network links between communicating agents

model

epoch

metrics
