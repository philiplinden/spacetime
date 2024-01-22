# Agents

Agents are the generic term for “things with clocks” in this context. It might be a satellite, rover, ground station, etc. This section aims to identify the minimal set of traits that are required for agents in this model. Hopefully this will tease out areas of overlap between different types of agents and a blueprint for the model will take shape.

## Attributes

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

The power (in dBm) of an agent’s receiver’s noise floor.

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
