# Intuitions for Relativity - Light

To begin, we develop some intuitions around the consequences of the speed of
light being constant. To narrow the scope and really dial in our understanding
of the fundamentals, consider the special case where all objects are moving at
constant velocity, far from any gravity well (aka, _special_ relativity). Let's
follow the logic and see where it takes us.

## Inertial reference frames

The first key insight is to approach the problem with the understanding that
motion is relative to the _inertial reference frame_ of the _observer_. In any
situation, we can always choose an inertial reference frame in which one object
is at rest and everything else is moving relative to it.

```admonish example
`Ash` is a train conductor, and `Brock` is waiting for the train to arrive at
the station. The train is moving at speed $v$ relative to the station.
```

**From `Ash`'s perspective:**
- The train is at rest.
- The station is moving towards the train at speed $v$.

**From `Brock`'s perspective:**
- The train is moving towards the station at speed $v$.
- The station is at rest.

`Ash` and `Brock` experience the world from their respective inertial reference
frames. Each of their observations are true, and they are all valid.

## Measuring time

The time experienced by an observer at rest is called its _proper time_.

Consider a perfect "light clock" that each observer watches. The light clock
consists of a photon that bounces between two mirrors once per second. Since the
speed of light is constant in all inertial reference frames, an observer
watching their clock will always observe the photon moving at the speed of
light, $c$.

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
```

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

Light clock `A` is moving at speed $v$ relative to `Brock`, but he observes the
photon inside to be moving at speed $c$, not $c+v$. How can this be?

```admonish tip
It is natural to pause at this point and ask "how can this be true?" or "why is
the speed of light constant?"---stop yourself. For a moment, accept that it is
true and instead we should ask **"what must happen for this to be true?"** Trust
the process. 🙏
```

## The speed of light is constant

Remember, the first postulate of special relativity is that the speed of light
is constant in all inertial reference frames. Let's follow the math and see what
happens when we assume all of the observations are true.

$$
\text{speed} = \frac{\text{distance}}{\text{elapsed time}}
$$

```admonish tip
This effect is the origin of the [twin paradox](https://www.youtube.com/watch?v=GsMqCHCV5Xc).
```
