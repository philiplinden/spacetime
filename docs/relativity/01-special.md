# Light Speed, Reference Frames & Time

To begin, we develop some intuitions around the consequences of the speed of
light being constant and that all velocities are measured relative to some
frame of reference (aka, _relativity_).

To narrow the scope and really dial in our understanding of the fundamentals,
consider the special case where all objects are moving at constant velocity, far
from any gravity well (aka, _special_ relativity). This is where Einstein
started, too.

Let's follow the logic and see where it takes us.

## Einstein's postulates

Einstein began with two postulates
([source](https://openstax.org/books/university-physics-volume-3/pages/5-1-invariance-of-physical-laws)):

1. **The laws of physics are the same in all inertial reference frames.**
2. **Light travels in a vacuum with the same speed $c$ in any direction in all
   inertial frames.**

The "laws of physics" in our analysis are then only those that satisfy the first
postulate. This also means that there is no "special" inertial reference frame
with special properties. There is no "rest frame" or "absolute space", only
motion of one frame relative to another.

The second postulate states that the speed of light has the same definite
speed for any observer, regardless of the motion of the source.

So, like Einstein, we will try our best to forget about what we _think_ we know
and instead focus on the consequences of the postulates.

## Inertial reference frames

The first key insight is to approach the problem with the understanding that
motion is relative to the inertial reference frame of the observer.

```admonish cite
An **inertial reference frame** is a frame of reference in which a body at rest
remains at rest and a body in motion moves at a constant speed in a straight
line unless acted upon by an outside force.
([source](https://openstax.org/books/university-physics-volume-3/pages/5-1-invariance-of-physical-laws))
```

```admonish example
`Ash` is a train conductor, and `Brock` is waiting for the train to arrive at
the station. The train is moving at speed $v$ relative to the station.

**From `Ash`'s perspective:**
- The train is at rest.
- The station is moving towards the train at speed $v$.

**From `Brock`'s perspective:**
- The train is moving towards the station at speed $v$.
- The station is at rest.

`Ash` and `Brock` experience the world from their respective inertial reference
frames. Each of their observations are true, and they are all valid.
```

## Measuring time

The time experienced by an observer at rest is called its _proper time_.

Consider a perfect "light clock" that each observer watches. The light clock
consists of a photon that bounces between two mirrors once per second. We'll

[ insert visualization of a photon clock ]

Since the speed of light is constant in all inertial reference frames, an
observer watching their clock will **always** observe the photon moving at the
speed of light, $c$, in their own reference frame.

The physics of relativity happen regardless of what we use to measure time, but
this "light clock" isn't arbitrary. We will use the fact that we are measuring
how long it takes for a photon to travel a given distance to measure time.

Let's give a light clock to `Ash` and `Brock` and see what happens.

```admonish example
`Ash` is riding a train, and `Brock` is waiting for the train to arrive at the
station. The train is moving at speed $v$ relative to the station.

`Ash` holds a light clock called `A`. `Brock` also holds a light clock called
`B`. `Ash` and `Brock` both observe the speed of light in their own light clocks
and in each other's light clocks.

**From `Ash`'s perspective:**
- The station is moving towards the train at speed $v$.
- Light clock `A` is at rest.
- The photon in the light clock `A` is moving at speed $c$.
- Light clock `B` is moving towards the train at speed $v$.
- The photon in the light clock `B` is moving at speed $c$.

**From `Brock`'s perspective:**
- The train is moving towards the station at speed $v$.
- Light clock `A` is moving towards the station at speed $v$.
- The photon in the light clock `A` is moving at speed $c$.
- Light clock `B` is at rest.
- The photon in the light clock `B` is moving at speed $c$.
```

Light clock `A` is moving at speed $v$ relative to `Brock`, but he observes the
photon inside to be moving at speed $c$, not $c+v$. How can this be?

Let's use the first postulate of special relativity and follow the math to see
how all of the observations could be true.

```admonish help
It is natural to pause at this point and ask "how can this be true?" or "why is
the speed of light constant?"---stop yourself. For a moment, accept that it is
true and instead we should ask **"what must happen for this to be true?"** Trust
the process. 🙏
```

## The speed of light is constant

```admonish important
**The distance a photon travels in a given amount of time is constant in every
inertial reference frame.**

$$
\text{speed}
= \frac{\text{distance traveled}}{\text{elapsed time}} =
\frac{\text{meters}}{\text{seconds}}
\\
\quad
\\
c = 299792458 \text{ m/s}
$$
```

This statement is
[more complicated](https://www.youtube.com/watch?v=ZbGxXyqlhbU&t=187s) than it
seems, but that's a tale for another time.

- $c$ is defined as the distance traveled by a photon in 1 second.
- $c$ is _not a measurement_, it is a _definition_. In actuality, the definition
  of $1 \text{ meter}$ is defined as the distance light travels in
  $\frac{1}{299792458}$ seconds.
- Science has accepted that this value, $299792458 \text{ m/s}$, is what we have
  decided to call the speed of light.

It is key to pin the definition of $c$ this way because as we will see later,
distance and time are _not absolute_, they are _relative_. The time or distance
can change depending on the observer's reference frame, but **distance traveled
by a photon divided by elapsed time is always $c$.**
