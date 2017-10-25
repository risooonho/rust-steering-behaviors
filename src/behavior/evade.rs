use nalgebra::{distance_squared, Point3};
use super::super::{Steerable, SteeringAcceleration, SteeringAccelerationCalculator,
                   SteeringBehavior};
use alga::general::Real;
use alga::general::AbstractModule;

#[derive(Builder)]
pub struct Evade<'a, T>
where
    T: 'a + Real,
{
    /// Common behavior attributes
    pub behavior: SteeringBehavior<'a, T>,
    /// Target to evade
    pub target: &'a Steerable<T>,

    pub max_prediction_time: T,
}


impl<'a, T: 'a + Real> SteeringAccelerationCalculator<T> for Evade<'a, T> {
    fn calculate_real_steering<'b>(
        &self,
        steering_acceleration: &'b mut SteeringAcceleration<T>,
        owner: &'b Steerable<T>,
    ) -> &'b mut SteeringAcceleration<T> {
        let square_distance = distance_squared(
            &Point3::from_coordinates(*self.target.get_position() - *owner.get_position()),
            &Point3::origin(),
        );
        let square_speed = distance_squared(
            &Point3::from_coordinates(*owner.get_linear_velocity()),
            &Point3::origin(),
        );
        let mut prediction_time = self.max_prediction_time;
        if square_speed > T::zero() {
            let square_prediction_time = square_distance / square_speed;
            if square_prediction_time < self.max_prediction_time * self.max_prediction_time {
                prediction_time = square_prediction_time.sqrt();
            }
        }

        steering_acceleration.linear = *self.target.get_position();
        steering_acceleration.mul_add(
            SteeringAcceleration::new(*self.target.get_linear_velocity(), T::zero()),
            prediction_time,
        );
        steering_acceleration.linear -= *owner.get_position();
        steering_acceleration.linear = steering_acceleration.linear.normalize();
        steering_acceleration.linear = steering_acceleration.linear.multiply_by(
            match self.behavior.limiter {
                Some(a) => -(*a).get_max_linear_acceleration(),
                None => T::one(),
            },
        );
        steering_acceleration.angular = T::zero();
        steering_acceleration
    }

    fn is_enabled(&self) -> bool {
        self.behavior.enabled
    }
}
