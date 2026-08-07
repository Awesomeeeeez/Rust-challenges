use crate::traffic_light::TrafficLight;

pub trait Next {
    fn next(&self) -> TrafficLight;
}