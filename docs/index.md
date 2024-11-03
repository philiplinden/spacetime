# spacetime: a real(istic) time simulator

`spacetime` is a hobby project by
[Philip Linden (me)](https://github.com/philiplinden) that explores the nature
of time synchronization between terrestrial and lunar actors with particular
interest in the discovery of emergent characteristics from dynamic
networks of cislunar actors operating in cislunar conditions.

## Goal

This project aims to use simulations to explore how centralized and
decentralized PNT service network topologies evolve in a growing lunar
ecosystem. Modeling network topologies representative of near-term lunar
missions and large future populations of cislunar actors will predict the
relative performance, “critical mass” of assets required for service, and
coverage of decentralized PNT services and/or GNSS-like beacons providing PNT to
lunar missions. There is an abundance of prior art describing optimal orbit
configurations for such systems that can also be evaluated in this way.

## Motivation

Recent advancements in space technologies have prompted a surge in lunar
missions, both crewed and uncrewed. Such an influx demands scalable,
commercially-accessible Positioning, Navigation, and Timing (PNT) frameworks for
the development of a cislunar economy. In order to bring PNT infrastructure to
the lunar ecosystem and have it be as ubiquitous and as useful as Global
Navigation Satellite Systems (GNSS) are in the interoperable Space Service
Volume (SSV), there needs to be accurate, traceable and accessible timing and
ranging infrastructure that is also resilient, reliable and flexible. NASA’s
LunaNet and ESA’s Moonlight are two major initiatives to promote
interoperability and connectivity in cislunar space by providing a common
communications framework and standards. Lunar constellations equivalent to
terrestrial GNSS are one approach to delivering a cislunar PNT but it is not the
only solution. Peer-to-peer networks of satellites with precision timekeeping
may serve as an alternative method of implementing a PNT service to traditional
GNSS constellations.

This project will use simulation and system modeling to compare satellite
network topologies using metrics known from the current PNT solution such as
accuracy, availability, continuity and integrity, in addition to costs, timeline
and technology development requirements of implementing each system in a
cislunar context. The goal is to develop the proposed solution to a level mature
enough to predict the system’s performance relative to the number and
distribution of interconnected assets, and quantitatively demonstrate that our
approach becomes more robust and performant as it scales to service the
anticipated demands of a thriving lunar ecosystem. The study will also consider
specific lunar PNT user needs and infrastructure combination opportunities, as
well as requirements for Earth / Earth Orbit systems to be usable with minimum
changes for lunar applications.

### Diverse cislunar ecosystems and infrastructures are inevitable

It is likely that several independent PNT services could emerge on different
parts of the spectrum. This allows actors to maintain closed PNT utilities or to
offer services for a self-sustaining, monetizable, commercially
owned-and-operated lunar infrastructure. Critically, public and private PNT
utilities may coexist under this paradigm, like how a single transponder can
access both terrestrial open-air radio and encrypted radio channels. In essence,
this philosophy aims to nurture a resilient PNT ecosystem that accommodates both
public and private ventures. Through a credibly neutral protocol for
timekeeping, bad actors would not only have difficulty manipulating the service,
but they may use this infrastructure themselves and even work to support its
canonization.

### Interoperability requires a common temporal reference frame

In the design of a lunar PNT system, one important consideration is the
definition of a reference frame to allow for absolute position. This time
transfer and relative position concept could be used to define a network of
realization points (fixed points for the reference frame) to assist in the
realization of a Lunar Reference Frame. These lunar realization points would be
located on the near side of the Moon and equipped with e.g., laser
retroreflectors for accurate ranging from Earth by the existing Lunar Laser
Ranging (LLR) stations. Like GNSS, passive receivers can obtain the time and
position in reference to the PNT node’s position by observing the transmitted
signal so long as the receiver’s clock is synchronized to the node.
