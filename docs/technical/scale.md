# scale
One of the difficult parts of modeling space systems is scale.

1. Space is huge, spacecraft are tiny.
2. We must track of time on the order of nanoseconds (or better!) to model real systems.
3. The numerical precision in our code is limited, and we're pushing those limtis here.

Even if we forget the idea of visualizing the system, the math still depends on the precision of these very large and
very small values.

## shrinkage
Can't we just shrink values down by some scale factor like $10^-6$ and carry on? Maybe.

Scaling down the numbers doesn't get rid of the precision problem since the small deltas between values are important.