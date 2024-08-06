# 2D, not 3D

In 2D physics-space, rendering-space, and screen-space are basically the same thing. We can hand-wave away translations
therein and all the complexities of visualizing information about a 3D volume in 2D screen-space. 

2D simulation space was selected mainly because it simplifies visualizations of information like gravitational potential
over large areas and it makes the use of compute shaders simpler as well. For example, offloading those gravity field
calculations to the GPU with a shader and then recovering that information back on the CPU later.
