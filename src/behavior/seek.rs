use nalgebra::Vector3;
use super::super::SteeringBehavior;
use super::super::{SteeringAcceleration, SteeringAccelerationCalculator};
use alga::general::Real;
use alga::general::AbstractModule;
use num_traits::identities::Zero;

/// Seek behavior calculates the maximum linear valocity to reach the target location
pub struct Seek<'a, T: 'a + Real> {
    /// common steering behavior attributes
    pub behavior: SteeringBehavior<'a, T>,
    /// steering target
    pub target: Vector3<T>,
}

impl<'a, T: Real> SteeringAccelerationCalculator<T> for Seek<'a, T> {
    fn calculate_real_steering<'b>(self: &mut Self,
                                   steering_acceleration: &'b mut SteeringAcceleration<T>)
                                   -> &'b mut SteeringAcceleration<T> {
        steering_acceleration.linear = (self.target - *self.behavior.owner.get_position())
                                           .normalize()
                                           .multiply_by(self.behavior
                                                            .limiter
                                                            .get_max_linear_acceleration());
        steering_acceleration.angular = T::zero();
        steering_acceleration
    }

    fn is_enabled(self: &Self) -> bool {
        self.behavior.enabled
    }
}
