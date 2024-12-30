# Gravity, Curved Space, & Curved Time

Now let's bring our understanding of relative to the general case where there
could be a gravity well nearby (aka _general_ relativity).

General Relativity is a theory of gravity that describes how gravity affects the
curvature of spacetime. It builds on the principles of Special Relativity,
following the natural consequences of the constancy of the speed of light
further and leading to the emergence of gravity and curved spacetime.

A key insight that will help us later is to understand that this definition
holds true _even if we morph the reference frame_. So on the grid, an object
would still move from point A to point B with respect to the reference frame.

<iframe
    width="1290" height="735"
    src="https://www.youtube.com/embed/x1dGfxBdDlM"
    title="Examples Of Nonlinear Two Dimensional Transformations"
    frameborder="0" allow="autoplay; clipboard-write; encrypted-media; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen>
</iframe>

## Space-time coordinates

Instead of thinking only in spatial coordinates, we can think in space-time
coordinates. One axis is movement through time, and the other is movement
through space. This coordinate system is called
[Minkowski space](https://en.wikipedia.org/wiki/Minkowski_space). Before
continuing, it is critical to remember that in this coordinate system we think
of an object moving at _any_ speed as having a larger component along the
`space` axis. So forget the _direction_ of motion for now.

Let's first consider the spatial component for our example. From `Brock`'s
perspective, he is at rest---his space-time vector has zero space component.
`Ash` is moving at speed $v$ relative to `Brock`, so his space-time vector has
a component along the `space` axis proportional to $v$.

```text
   * Brock
   *------* Ash
+---------------→ Space
```

Now we consider the time component---this is where the magic happens. Let's
"tick" time forward by one instant. From the last section, we know that `Ash`
will experience slightly less time than `Brock`.

```text
Time
▲     Brock
│  *  (1,11)
│  |
│  |        Ash
│  |      * (8,8)
│  |     /
│  |    /
│  |   /
│  |  /
│  | /
│  |/
│  * (1,1)
+---------------→ Space
```

Working out the math, we find that the length of both space-time vectors is
exactly $c$.
