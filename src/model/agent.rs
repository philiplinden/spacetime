/*!
 * An agent that lives in the world.
 *
 * An agent is a single entity in the simulation. Agents sense the world, but
 * their perception may not exactly match the world's "true" state---its
 * sensors might not be ideal---in order to form an understanding of its own
 * state. Agents are able to interact with the world and with other agents.
 * 
 * # Traits
 * Agent behavior is determined by its traits. An agent has one or more traits
 * from each of the following categories:
 *   - Domain
 *   - Bus
 *   - Radio
 *   - Clock
 * 
 * ## Domain
 * An agent's _domain_ is the environment it was designed to live in. The
 * domain can be `Space` or `Ground`. Ground agents are attached to the surface
 * of a body. Ground agents may traverse across a body's surface. Space agents
 * have orbital trajectories around one or more bodies.
 * `Rover`, 
 */
