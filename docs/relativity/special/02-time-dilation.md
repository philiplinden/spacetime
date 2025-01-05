# Special Relativity

## Time Dilation

The time experienced by an observer in its inertial frame is called its _proper_
_time_. Something interesting happens when we compare the proper time of a body
at rest to the proper time of a body in motion within the same inertial frame
that teaches us an important lesson about the nature of time.

```admonish cite
Time intervals have different values when measured in different inertial frames.

Time dilation is the lengthening of the time interval between two events for an
observer in an inertial frame that is moving with respect to the rest frame of
the events (in which the events occur at the same location).

The proper time interval $\Delta \tau$ between two events is the time interval
measured by an observer for whom both events occur at the same location.

([source](https://openstax.org/books/university-physics-volume-3/pages/5-3-time-dilation))
```

Consider a "light clock" that consists of a photon that bounces between two
mirrors set a distance $D$ apart. We count one "tick" of the clock when the
photon makes one full round trip. Since the speed of light is constant in all
inertial reference frames, an observer will **always** observe the photon moving
at the speed of light, $c$, regardless of the motion of its source.

```admonish example
An astronaut observes a photon in the light clock on a spaceship moving at a
constant velocity, $v$, relative to the Earth. An astronomer on Earth also
observes the same photon in the light clock. Both observers measure the time it
takes for the photon to make one full round trip.

- The astronaut observes the photon moving at exactly the speed of light, $c$.
- The astronomer observes the photon moving at exactly the speed of light, $c$.
- The astronaut and astronomer disagree on the time it takes for the photon to
  make one full round trip in the light clock.
  - The astronaut observes the photon completing one round trip in
    $\Delta \tau$.
  - The astronomer observes the photon completing one round trip in
    $\Delta t$.

**Problem**: Whose round-trip duration measurement is correct?

![openstax example](https://openstax.org/books/university-physics-volume-3/pages/5-3-time-dilation)

**Solution**: The astronaut and astronomer are both correct! It's not a trick
question, this time---we can solve it with 8th-grade math and disciplined logic.

In the astronaut's inertial frame:
- The photon moves at speed $c$.
- The light clock is at rest so the photon travels the distance between the
  mirrors in the time it takes to complete one round trip. This is our intuitive
  understanding of time in the everyday sense.
- The time it takes for the photon to make one full round trip is $\Delta \tau$.

In the astronomer's inertial frame:
- The photon moves at speed $c$.
- The light clock is moving relative to the astronomer so the photon travels a
  longer distance in the time it takes to make one full round trip in the clock.
  - The photon travels sideways a distance $v \Delta t$ in addition to the
    distance it travels up and down between the mirrors.
  - Despite moving additional distance in this reference frame compared to the
    astronaut's frame, the photon travels at speed $c$ in both reference frames.
  - This means that in the astronaut's frame, the photon travels _more distance_
    while moving at the _same speed_.
- The only way this can be true is if the elapsed time experienced by the
  astronaut, $\Delta \tau$, is longer than the elapsed time experienced by the
  astronomer, $\Delta t$, when observing the same photon in the astronomer's
  reference frame.
```

We see here that both of Einstein's postulates are satisfied, and we have not
violated any laws of physics. Yet two observers measured the photon taking
different durations to make the same trip in space. How can this be?

```admonish help
Remember that physics and science do not describe the _truth_, they describe
math and logical axioms that are consistent with the observed data. As
counterintuitive as it may be,
[special relativity overwhelmingly agrees with scientific observations](https://en.wikipedia.org/wiki/Tests_of_special_relativity).
```

In the astronomer's reference frame, the photon moves across more space over the
course of one "tick" of the clock. The only way the speed of the photon, $c$,
can be the same in all reference frames and yet travel across more distance in
one reference frame than in another is if the elapsed time experienced by a
moving object, $\Delta \tau$, "dilates" compared to the elapsed time
experienced by an observer at rest in the reference frame, $\Delta t$.

![time dilation](./assets/TimeDilationDemo.gif)
