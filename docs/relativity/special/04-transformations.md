# Special relativity

In this section we'll see how relativity affects equations of motion for events
measured between inertial reference frames. The majority of this work is based
on the Relativity chapter in
[OpenStax University Physics Volume 3](https://openstax.org/books/university-physics-volume-3/pages/5-5-the-lorentz-transformation).

## Coordinate transformations between inertial reference frames

An _event_, $s$, is a location and time coordinate relative to an inertial
reference frame, $S$. For simplicity, we'll abbreviate the time derivatives as
$\dot{s}$ and $\ddot{s}$. I've written it out below, expanded out into the $x$,
$y$, and $z$ spatial components to remind us that these are vectors in 3D space.

$$
\begin{align*}
\text{position\quad}& s = \begin{bmatrix} x \\ y \\ z \end{bmatrix}\\
\text{velocity\quad}& \dot{s} = \begin{bmatrix} \dot{x} \\ \dot{y} \\
\dot{z} \end{bmatrix} = \frac{d\vec{s}}{dt}(t)\\
\text{acceleration\quad}& \ddot{s} = \begin{bmatrix} \ddot{x} \\ \ddot{y} \\
\ddot{z} \end{bmatrix} = \frac{d^2\vec{s}}{dt^2}(t)
\end{align*}
$$

But what if we want to define the same event relative to a moving reference
frame, $S'$?

![moving reference frame](https://openstax.org/apps/archive/20241024.164013/resources/a2b3997dff7a717555902ca3279bc1723837f76d)

### Galilean Transformations

Suppose $S'$ is moving with respect to $S$ at velocity $\vec{v}$.

In Newtonian mechanics, the transformation between two inertial reference frames
is given by the Galilean transformations. This is the same thing you might have
done in high school physics or an engineering course.

$$
s(t) = s'(t) + vt
$$

The underlying assumption here is that **time $t$ is the same for observers in
$S$ and $S'$**. So the we can simply differentiate the position vector to get
the velocity and acceleration vectors for the event in the moving reference
frame.

$$
\begin{align*}
t &= t' \\
\text{and} \quad dt &= dt' \\
\text{therefore} \quad \dot{s} &= \dot{s}' + v\\
\ddot{s} &= \ddot{s}'
\end{align*}
$$

Observers in $S$ and $S'$ will measure the same acceleration for the event.
Because mass is unchanged by the transformation, $F=ma$ the force measured by
the observers will be the same and Newton's laws hold. All is well and you get
an $A+$ in physics class.

### Testing against Einstein's postulates

Let's see if Einstein's postulate that the speed of light is the same for all
inertial reference frames holds when applying the a Galilean transformation.

We can test this by calculating the speed of a photon in $S$ and $S'$.

$$
\begin{align*}
\text{let} \quad \dot{s} = c, \quad\dot{s}' = c\\
\text{if}& \quad \dot{s} = \dot{s}' + v\\
\text{then it must be true that}& \quad c = c + v\\
\end{align*}
$$

This is **not** true, so the Galilean transformation violates the speed of light
postulate!

The issue comes from the assumption that time is the same for observers in $S$
and $S'$, which we know from [the previous chapter](02-time-dilation.md) is not
true. In most cases, the relative motion $v$ is much smaller than the speed of
light, $c$, so the Galilean transformation is a good approximation. But if $v$ is
significant compared to $c$ or we want to be very precise over long distances,
we need a more accurate transformation that accounts for the effects of
relativity.

### Lorentz Transformations

Good news, someone already figured this out! The Lorentz transformations are a
set of equations that describe how to transform coordinates between two
inertial reference frames that are in relative motion and are consistent with
the speed of light postulate.

![moving reference frame](https://openstax.org/apps/archive/20241024.164013/resources/a2b3997dff7a717555902ca3279bc1723837f76d)

Recall our [previous example](02-time-dilation.md) with the astronaut and the
astronomer. Let's say the astronomer's frame of reference is $S$ and the
astronaut's frame of reference is $S'$, moving at velocity $v$ relative to $S$
and the $x$ axis is the direction of motion.

The astronomer observes the origin of $S'$ at time $t$. The astronomer also
observes the displacement of a photon in the astronaut's light clock from the
origin of $S'$ to be $x'\sqrt{1 - \frac{v^2}{c^2}}$.

$$
\begin{align*}
x' &= \gamma (x - vt)\\
t' &= \gamma (t - \frac{vx}{c^2})
\end{align*}
$$
