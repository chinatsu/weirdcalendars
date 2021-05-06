pub mod decimal_time;
pub mod new_earth_time;
pub mod swatch_internet_time;

pub trait Time<T> {
    fn now() -> Self;
}