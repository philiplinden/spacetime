# potential volumes

_A heightmap, but instead of encoding 3D data to a 2D colormap let's encode a gradient field into a 3D volume of values._

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

## texture maps
This is relevant, I swear.

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

## my idea
Back on topic.

Recall the rubber sheet analogy for spacetime: we imagine curved 3D spacetime in 2D as a flat sheet that is deformed in
the third dimension. Now recall how heightmaps encode elevation: a 2D image contains spatial information in pixel
coordinates and intensity (elevation) as the grayscale value, then drape RGB color information on top. See where I'm
going with this?

What if we extend the heightmap concept the same way we extend the rubber sheet analogy. The heightmap _is_ the rubber
sheet. To represent potentials, consider a _volume_ with intensity values at each coordinate. This is no different than
the opacity of volumetric fog in a video game.

Now that we're thinking in terms of video games, what if we leverage other advanced techniques originally developed to
change _colors_ and rewire it to represent potential instead?

### changing potential over time
One snag here is that Earth and Luna are moving with respect to each other over time. Even if we restrict our scope
to just Earth and Luna (excluding influences from Sol, Jupiter, etc.) we still have to contend with the potential field
changing over time.

Maybe we can leverage techniques normally used for things like [realtime heightmap deformation](https://gilzoide.itch.io/raise-and-shine),
maybe [using the GPU](https://developer.nvidia.com/gpugems/gpugems3/part-i-geometry/chapter-1-generating-complex-procedural-terrains-using-gpu)?
I suspect this will be an expensive thing to keep track of, but it's a solved problem already.

[^1]: Fraknoi, Andrew, David Morrison, and Sidney Wolff. 2022. “24.4 Time in General Relativity - Astronomy 2e | OpenStax.” OpenStax. March 9, 2022. https://openstax.org/books/astronomy-2e/pages/24-4-time-in-general-relativity.

[^2]: 3D-Mapper. 2022. “Heightmaps and Textures.” https://3d-mapper.com/heightmaps-and-textures/. Accessed 21 Jan. 2024.

