# potentials

For this project we care about _time_, but physics (especially gravity) play a part in timing for objects in cislunar
space. Let's take a look at the effects celestial bodies will have on time, starting from first principles and
eventually thinking about how we can make these calculations on cislunar scales with potentially thousands of entities
on their own trajectories between Earth and Luna.

## gravitational potential
_Potential_ is how we represent stored energy in an object "at rest," with our definition of "at rest" basically meaning
kinetic energy is zero. In our case, we're most interested with gravitational potential, i.e. the potential energy of an
object at rest caused by gravity.

All but a lucky few humans have lived their entire lives in a tiny sliver on and around Earth's surface. From the
deepest ocean to the tallest mountain, gravity feels _basically_ the same. But what happens when our object goes far
from Earth, to the Moon or beyond? Does gravity change between here and there? (yes) What even _is_ gravity? (well...)

### newtonian physics
[Newtonian physics tells us](https://openstax.org/books/college-physics-2e/pages/7-3-gravitational-potential-energy)
gravitaional potential ($PE$) is the work done against gravity ($g$), so it's porportional to changes in height ($h$).
This is naive, but it hints at deeper intuitions about gravity.

$$
\Delta PE = mg\Delta h
$$

But for Newton, gravity just... is. Look up the value in a table based on measurements someone went through a great deal
of effort to record. What is it? `9.7803267715 m * s^-2`. Okay, but what IS it? To answer this question, we need a more
detailed description of gravitational potential.

### general relativity

General Relativity (GR) gives us the mathematical vocabulary to describe gravitational potential with curved spacetime.

The classic example here is imagining gravity as a big weight in the middle of a rubber sheet. The mass of large objects
changes the curvature of spacetime so light has to take the "long way around", but the speed of light is constant so
[time has to slow down](./time.md) to make up the difference.

![rubber sheet analogy](https://openstax.org/apps/archive/20231109.173216/resources/0531ebe01f0822745bd09fb9caa337b514a5f0a4)[^1]

!!! abstract
    Read more about Relativity on OpenStax with the [Astronomy 2e](https://openstax.org/books/astronomy-2e/pages/24-2-spacetime-and-gravity).

The important takeaway from this discussion of relativity is that the _potential_ is represented by the intensity of
that curvature we imagined before. An object "rolls downhill" along this curvature, and the slope of that curve is
gravity.

## algorithms

### barnes-hut algorithm
The [Barnes-Hut Algorithm](http://arborjs.org/docs/barnes-hut) is a way to use a 
[quad-tree](https://jimkang.com/quadtreevis/) for n-body problems.[^3] In short, far away bodies are lumped together and
gravitation is approximated using the lump's center of mass rather than all $n$ particles and not bothering to do any
computations on quadrants without any bodies. This reduces an $O(n^2)$ problem to an $O(n \log{n})$ problem. This is
also a great use case for GPU-driven parallel computations.

That's great, but this is overkill. The mass of any spacecraft is negligible with respect to any planet, moon, and star
in the solar system. Once you leave out all the spacecraft bodies, we're left with $n=4$ at most (Sol, Earth, Luna,
Jupiter) for our computations. A "true" n-body simulation yields pretty much the same results if we assume all
spacecraft are massless for gravity calculations, so it's not worth it to simulate the mass of every particle. Given the
distances between the remaining massive bodies ($n=4$), a Barnes-Hut algorithm ($O(n\log{n}) = 4 \log{4} \approx 2.41)
is still more efficient than brute-force ($O(n^2) = 4^2 = 16), but not as significantly as the gains when $n$ is very
large. For example, if $n=100$ BH needs $200$ computations compared to brute force's $10,000$ computations. In the scope
of modern CPUs, the gains at $n=4$ from Barnes-Hut are marginal.

I think there is a case to use Barnes-Hut elsewhere, but I don't think we need an n-body simulation here.

### almanac look-ups
The fastest calculation is no calculation at all.

If we only consider potential fields from celestial bodies, we can use very accurate and precise almanacs to *look up*
the exact positions of each body at a given time. Not only is this fast and easier, but I'd wager its more accurate than
we could ever hope to achieve calculating it ourselves.

## using the gpu
### texture maps
A [heightmaps](https://en.wikipedia.org/wiki/Heightmap), or more broadly a "texture map", is a technique in computer 
graphics where information across a discretized area is stored as an image. Instead of expensive geometry edges and
vertices carving out every nook and cranny in 3D space, graphics engines interpret the image data from the grid and
react according to the image value instead of the geometry. Essentially it's an effective way to compress detailed
information for a discrete grid.

A classic example of this is with digital elevation maps, where image coordinates represent spatial coordinates and
pixel value represents elevation.

[heightmaps with color are basically the same as what we want](https://3d-mapper.com/wp-content/uploads/2022/09/heightmap-and-textures-3d-map.png)[^2]

In computer graphics, this technique evolved to use color channels available in image formats to efficiently manage
[even more characteristics](https://cgcookie.com/posts/normal-vs-displacement-mapping-why-games-use-normals) without
increasing the detail of the object models themselves. This is especially useful for light transport calculations and
other complicated situations where fine details matter. 

![normal map](https://s3.amazonaws.com/cgcookie-rails/wp-uploads/2017/06/diagram_01.jpg)

There are even more advanced techniques like [parallax mapping](https://learnopengl.com/Advanced-Lighting/Parallax-Mapping).
It's a rabbit hole for sure.

Recall the rubber sheet analogy for spacetime: we imagine curved 3D spacetime in 2D as a flat sheet that is deformed in
the third dimension. Now recall how heightmaps encode elevation: a 2D image contains spatial information in pixel
coordinates and intensity (elevation) as the grayscale value, then drape RGB color information on top. See where I'm
going with this?

What if we extend the heightmap concept the same way we extend the rubber sheet analogy. The heightmap _is_ the rubber
sheet. To represent potentials, consider a _volume_ with intensity values at each coordinate. This is no different than
the opacity of volumetric fog in a video game.

But how do we contend with a potential field that evolves over time as celestial bodies move?

### shaders
Now that we're thinking in terms of video games, let's leverage other advanced techniques originally developed to
change _colors_ and rewire it to represent potential instead.

Enter: [compute shaders](https://webgl-shaders.com/gravity-example.html). Rather than using _images_, we can implement a
_shader_ to do simple math based on our world coordinates and the bodies in the world. Shaders are generally used in
computer graphics, and a particularly interesting case is GPU-accelerated "particle systems". In our use case, we don't
care about what anything looks like, or the intricate details of a mesh. We want to leverage the shader to compute the
total potential at any given point in our field. These computations are independent, so they can be run in parallel and
accelerated by the GPU. The results are then given to the next frame of the simulation so when one of our bodies needs
to know the total potential at its position, it simply looks up the shader-computed value at its coordinate.

This method has been used for real-time fluid simulations ([example 1](https://youtu.be/69cRIreyb8g), 
[example 2](https://www.youtube.com/watch?v=7BQgDPup7lE)) and n-body gravity simulations 
([example 1](https://webgl-shaders.com/gravity-example.html), [example 2](https://www.youtube.com/watch?v=qix0rzIaiIs))
so a potential field should be a piece of cake.

...right?

[^1]: Fraknoi, Andrew, David Morrison, and Sidney Wolff. 2022. “24.4 Time in General Relativity - Astronomy 2e | OpenStax.” OpenStax. March 9, 2022. https://openstax.org/books/astronomy-2e/pages/24-4-time-in-general-relativity.

[^2]: 3D-Mapper. 2022. “Heightmaps and Textures.” https://3d-mapper.com/heightmaps-and-textures/. Accessed 21 Jan. 2024.
