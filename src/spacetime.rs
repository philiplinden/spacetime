/*!
 * This module handles the warping of spacetime. This includes gravity and time dilation.
 * 
 * The time dilation of an accelerating object is given by
 *
 *      tau(t) = (c / g) arcsinh(g * t / c), where
 *
 *          t -> elapsed proper time
 *          c -> speed of light
 *          g -> acceleration
 * 
 * For other relativistic effects, we may need to reference the Lorentz factor given by
 *
 *      gamma = 1 / sqrt(1 - v^2 / c^2), where
 *
 *          v -> speed of moving observer
 *          c -> speed of light
 * 
 * Time dilation can also be expressed in terms of Lorentz factor:
 * 
 *      dt = gamma * dtau
 * 
 * Note that if we integrate over time and substitute v -> (g * t), we arrive at the same definition as before. However,
 * using the more complicated form with acceleration lets us account for the accelerations from gravity when an object
 * is near a large body.
 */
