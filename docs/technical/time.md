# frames of reference
## coordinate time
The time as observed by the coordinate frame itself. This is a "god's-eye view" of the time in the system we are
analyzing. Basically, it represents a perfect clock held by the world/universe/simulation coordinate frame's observer.
Usually Earth-centric systems use UTC or TAI as the Coordinate Time.

Formally, the coordinate time is the time that would be read on a hypothetical "coordinate clock" situated infinitely
far from all gravitational masses, and stationary in the system of coordinates.
## proper time
The time as observed by a perfect clock attached to an object's local body frame. The body frame is a coordinate frame
rigidly attached to the object. Whenever the object observes the time, it observes the local Proper Time.

In most situations, Proper Time and Coordinate Time are the same. The distinction between Proper Time and Coordinate
Time is only relevant when objects are subjected to large accelerations or move across lumpy spacetime (like going from
sea-level on Earth to standing on the Moon), and the effects of General Relativity become significant.

## system time
So far we've only talked about the hypothetical, perfect clocks that describe the physics. But we actually want to
observe these times ourselves with real clocks. Unfortunately, no clock is perfect or ideal. The _System Time_ is the
real clock's observation of the local Proper Time. We'll explore System Time later when we learn more about
[Allan Variance](https://en.wikipedia.org/wiki/Allan_variance).

# relativity & time dilation
Let's apply our knowledge of Special Relativity (SR) and General Relativity (GR) to understand the phenomenon of time
dilation.

**Special Relativity** describes spacetime far from a significant gravity field, and explains how a stationary observer
perceives time passing slower for a moving observer when the relative velocity between them is very large. Relative
velocity is the distance traveled over time taken. Since the speed of light is constant, kinetic energy is working to
"slow down" the proper time with respect to coordinate time to keep the equation valid. This is special relativity at
work.

![of course there is an xkcd for this](https://imgs.xkcd.com/comics/recipe_relativity.png)[^1]

**General Relativity** deals with gravity, and explains how time slows down in the presence of a large
[gravitational potential](./potentials.md). Scientists have sent ultra-precise clocks into the void beyond Earth and
proved that proper time is slower for observers closer to the center of the gravity well. This is general relativity at
work.

![xkcd strikes again](https://www.explainxkcd.com/wiki/images/thumb/e/ed/bendy_2x.png/437px-bendy_2x.png)[^2]

It's whack, but the math checks out and matches real observations. If you're still not convinced, look to
[time_dilation.ipynb](../notebooks/time_dilation.md) for a worked out example.

# precision
(wip)


[^1]: Image credit: Randall Munroe, 2023. _Recipe Relativity_. https://xkcd.com/2767/

[^2]: Image credit: Randall Munroe, 2022. _Bendy_. https://xkcd.com/2706/