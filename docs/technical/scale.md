One of the difficult parts of modeling space systems is scale.

1. Space is huge, spacecraft are tiny.
2. We must track of time on the order of nanoseconds (or better!) to model real
   systems.
3. The numerical precision in our code is limited, and we're pushing those
   limits. ([Example](./notebooks/time_dilation.md))

Even if we forget the idea of visualizing the system, the math still depends on
very fine precision of these very large quantities.

Or that's what I assume. Let's pick apart this idea.

# scale factor
Can't we just shrink values down by some scale factor like $10^-6$ and carry on?
Maybe.

Scaling down the numbers doesn't get rid of the precision problem since the
small deltas between values are important.

# coordinate frames
coordinates are how we describe positions of entities. Simple matrix math and
rotations can translate a position from one coordinate frame to another.

For example, we can get a description of earth and moon motion with simple
transformations:
1. get origin of earth-centered inertial frame. this is attached to earth.
2. get origin of moon-centered inertial frame. this is attached to moon.
3. transform (1) and (2) into the earth-moon barycenter inertial frame. the
   result is that both earth and moon are rotating about the barycenter. we have
   the earth moon orbits now.

## global frames are the problem
Normal astronomical coordinate frames are based on observable datums: Earth, the
moon, Earth-moon barycenter, the sun, solar system barycenter, etc...

positions/velocities suffer the most here. We need precision both for large
values, and for small changes.

- Astronomical distances are huge. Even in ECI, coordinates at sea level are
  already at values like 6,000,000.0 meters.
- Pointing error: fractions of an arcsecond of error accumulate into kilometers
  of error across astronomical scales.
- Position/velocity error: at orbital speeds, a 1% error can lead to 100s of
  kilometers of error.
- Timing error: Ranging is a measurement of time-of-flight. At the speed of
  light, 1 nanosecond of timing error leads to 30 cm of position error.
- Relativity: Orbital speeds and altitudes (very large values for position and
  velocity) lead to very small timing differences of nanoseconds, which as we
  saw before, is significant.

## local frames are the answer
the traditional coordinate frames defined by nasa are not the _only_ coordinate
frames that exist.

we can chunk up our world space into many small frames. when an entity crosses
the boundary between chunks, transform coordinates into the new chunk's
coordinates.

this solves the scale problem by allowing the world to be very large, but
without requiring entity position coordinates to be represented by enormous
values. instead, whenever coordinates get too large in one frame, we transfer
the representation into a frame closer to the entity (but still rigidly attached
to the "world" space). 

analogy:
- instead of measuring my height as 73 inches, i denote it as 6 feet, 1 inch.
- instead of measuring map coordinates as 4000 meters East, 200 meters North
  from the arbitrary origin chosen by the cartographer, we can say a location is
  10 meters East, 1 meter North within grid square J4.

Implementing this idea of many small coordinate frames composing the world space
is not trivial, but it is a solved problem. The Bevy plugin
[big_space](https://github.com/aevyrie/big_space) addresses it by placingentities
in a `GridCell` in a large fixed-precision grid. In the `GridCell`, the entity's
`Transform` is relative to the center of that cell. If the entity enters a
neighboring cell, its coordinates are updated to reference the new cell's origin.
