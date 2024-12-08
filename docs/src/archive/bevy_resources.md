*Resources* in Bevy are used to store a single global instance of some data type independently of entities. A typical
use case for resources is to store configuration data and settings, which is one of the ways we'll use it here.

!!! abstract "Additional Information"
    Bevy's [official](https://bevyengine.org/learn/book/getting-started/resources/) and
    [unofficial](https://bevy-cheatbook.github.io/programming/res.html) books go into more detail about resources and
    how to use them.

# settings
All of the parameters that control the simulation's execution should be stored as resources. This ensures that every
part of the application will have access to these parameters, and the app will "hot reload" when the parameters change.

## visualization aids
### pause / resume
The app should allow the user to freeze the simulation at any instant. When the simulation is frozen, the user should
still be able to move the camera and interact with UI elements. 

### physics time step size
The app should allow the user to adjust the scale factor between the duration of a simulated time step and real time.
If the scale factor is less than 1 you get slow motion, greater than 1 you get fast-forward, and 1 makes the simulation
render in "real time".

Bevy has two relevant [built-in](https://bevy-cheatbook.github.io/builtins.html#resources) resources related to time:

    * `Time`: Global time-related information (current frame delta time, time since startup, etc.)
    * `FixedTime`: Tracks remaining time until the next
      [fixed update](https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html)