use alga::general::Real;

/// Interface to set limits  on linear and angular speed and acceleration of the agent
pub trait Limiter<T: Real> {
    fn get_zero_linear_speed_threshold(self: &Self) -> T;
    fn set_zero_linear_speed_threshold(self: &mut Self, threshold: T);
    fn get_max_linear_speed(self: &Self) -> T;
    fn set_max_linear_speed(self: &mut Self, linear_speed: T);
    fn get_max_linear_acceleration(self: &Self) -> T;
    fn set_max_linear_acceleration(self: &mut Self, linear_acceleration: T);
    fn get_max_angular_speed(self: &Self) -> T;
    fn set_max_angular_speed(self: &mut Self, angular_speed: T);
    fn get_max_angular_acceleration(self: &Self) -> T;
    fn set_max_angular_acceleration(self: &mut Self, angular_acceleration: T);
}
