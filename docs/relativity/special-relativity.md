# Intuitions for Special Relativity

Special Relativity is a theory of space and time that describes how space and
time are affected by motion. It follows the natural consequences of the
constancy of the speed of light in all reference frames. The _special_ in
special relativity refers to the fact that it is a special case of general
relativity: situations when objects are moving at constant velocities in a
vacuum.

[Special Relativity Intuitions](https://www.youtube.com/playlist?list=PLawLaqps30oBmdbw_D-AI1RQUoCO7Wr1K)

The **speed of light is constant in all inertial reference frames**, regardless
of the motion of the observer or the source of the light. This is a fundamental
postulate of special relativity.

## Inertial reference frames

The first key insight is to approach the problem with the understanding that
motion is relative to the _inertial reference frame_ of the _observer_. In any
situation, we can always choose an inertial reference frame in which one object
is at rest and everything else is moving relative to it.

```admonish example
`Ash` is a train conductor, and `Brock` is waiting for the train to arrive at the
station. The train is moving at speed $v$ relative to the station.
```
**From `Ash`'s perspective:**
- The train is at rest.
- The station is moving towards the train at speed $v$.

**From `Brock`'s perspective:**
- The train is moving towards the station at speed $v$.
- The station is at rest.

`Ash` and `Brock` experience the world from their respective inertial reference
frames. Each of their observations are true, and they are all valid.

The second key insight is to understand that the speed of light is constant for
every observer regardless of their motion or the source of the light.

```admonish note
Here we will ignore the effects of light passing through a medium for the sake
of brevity, but know that the physics still holds.
[This video](https://youtu.be/KTzGBJPuJwM?si=YbBgHIcYyOL3UuSA) provides an
excellent intuition about how the speed of light can be constant and also how
light moves "slower" through a medium.

**TLDW**; The speed of light is constant in any medium. The "slower" speed is a
sort of illusion caused by "phase kicks" between the light and the medium.
```

## The speed of light is constant

The first postulate of special relativity is that the speed of light is constant
in all inertial reference frames. 

Another way to think about this is to consider a perfect "light clock" that each
observer watches. The light clock consists of a photon that bounces between two
mirrors once per second. Since the speed of light is constant in all inertial
reference frames, an observer watching their clock will always observe the
photon moving at the speed of light, $c$.

Let's give a light clock to `Ash` and `Brock` and see what happens.

```admonish example
`Ash` is a train conductor, and `Brock` is waiting for the train to arrive at
the station. The train is moving at speed $v$ relative to the station.

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
me, it actually makes _more_ sense to approach the problem this way.
```

## The emergence of time dilation

Let's follow the math and see what happens when we assume all of the
observations are true.

$$
\text{speed} = \frac{\text{distance}}{\text{elapsed time}}
$$


```text
                distance traveled in one second
Brock  * 0 m
Ash    *--* 100 m
Light  *~~~~~~~~~~~~~~~~~~~~~~~~~~~~~* 300_000_000 m
```

$$
\begin{aligned}
c &= \frac{300,000,000 \text{ m}}{t_B \text{ s}} \\
c &= \frac{299,999,900 \text{ m}}{t_A \text{ s}} \\
\end{aligned}
$$

In order for the speed of light to be constant in both cases, the _elapsed time_
must be different for each observer!

$$
\begin{aligned}
\text{if} \quad 3.00\times 10^{8} \text{ m/s} &= \frac{300,000,000 \text{ m}}{t_B} \implies t_B = 1.00 \text{ s} \\
\text{then} \quad 3.00\times 10^{8} \text{ m/s} &= \frac{299,999,900 \text{ m}}{t_A} \implies t_A = 0.999999667 \text{ s} \\
\end{aligned}
$$

The time experienced by an observer at rest is called its _proper time_.
Remember that it an observer only ever sees themself at rest. The proper time is
always "normal" for the observer. (This is where the
[twin paradox](https://www.youtube.com/watch?v=GsMqCHCV5Xc) comes from.)
