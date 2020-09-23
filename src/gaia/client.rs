use crate::gaia::engine::Engine;

pub trait Client {
    fn update(&mut self, engine: &mut Engine);
    // fn draw<T>(&mut self, engine: &mut Engine<T>) where T: Client;
    unsafe fn draw(&mut self);
}