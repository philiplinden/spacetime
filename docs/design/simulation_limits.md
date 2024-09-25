# 2D or 3D?

In 2D physics-space, rendering-space, and screen-space are basically the same
thing. We can hand-wave away translations therein and all the complexities of
visualizing information about a 3D volume in 2D screen-space. 

2D simplifies visualizations of information like gravitational potential over
large areas and it makes the use of compute shaders simpler as well. For
example, offloading those gravity field calculations to the GPU with a shader
and then recovering that information back on the CPU later.

We can skip the complexities of general physics by using Nyx and other vetted
libraries. These libraries take complicated 3D interactions, NASA almanacs, and
other intracies into account under the hood. Nyx gives us the 3D engine "for
free" so to speak, so we can focus on the time dilation aspect of relativity
without having to worry about the complexities of general physics. However, we
are limited by the number of bodies we can simulate. 

The benefit of 2D physics is that it's easier to simulate. The benefit of 3D
physics is that it's more realistic.
