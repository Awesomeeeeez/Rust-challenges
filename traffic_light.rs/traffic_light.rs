use crate::traits::Next;

#[derive(Debug)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl Next for TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Red,
        }
    }
}