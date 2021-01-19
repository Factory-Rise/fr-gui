use sfml::window::Event;
use std::collections::HashMap;

pub struct Widget {
    events: HashMap<Event, Fn(_)>,
    texture: String,
    bounds: //TODO
}
