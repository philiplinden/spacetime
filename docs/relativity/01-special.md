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
2. **Light travels in a vacuum with the same speed $c$ in any direction in all**
   **inertial frames.**

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

In other words, in an inertial reference frame attached to an object that is
not accelerating, it is impossible to tell if the object is at rest or moving at
a constant velocity.

```admonish example
A blue dot is floating in interstellar space. The only object in sight is a red
dot. The red dot is moving at a constant angular velocity around the blue dot.

**Problem**: Is the blue dot moving around the red dot, or is the red dot moving
around the blue dot?

![inertial reference frame](./assets/InertialReferenceFrames.gif)

**Solution**: Trick question---the physics works the same way in both cases!
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

## Truth is in the eye of the ~~beholder~~ observer

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
  - The astronomer observes the photon completing one round trip in $\Delta t$.

**Problem**: Whose round-trip duration measurement is correct?

**Solution**: The astronaut and astronomer are both correct! It's not a trick
question, this time---we can solve it with 8th-grade math and disciplined logic.

In the astronaut's inertial frame:
- The photon moves at speed $c$.
- The light clock is at rest so the photon travels $2D$ in the time it takes to
  complete one round trip.
- The time it takes for the photon to make one full round trip is
  $\Delta \tau = \frac{2D}{c}$.

In the astronomer's inertial frame:
- The photon moves at speed $c$.
- The light clock is moving relative to the astronomer so the photon travels a
  longer distance in the time it takes to make one full round trip in the clock.
  - The photon travels sideways a distance $v \Delta t$ in addition to the
    distance $2D$ it travels up and down between the mirrors.
  - The total distance the photon travels can be found by pythagorean theorem.
    One side of a right triangle is $2D$ and the other side is $v \Delta t$. The
    hypotenuse is the total distance the photon travels over one tick, $s$.
    $$
    s = \sqrt{(2D)^2 + (v \Delta t)^2}
    $$


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

### Exploring time dilation
