use bevy::prelude::{Color, Assets, Mesh};

mod simple_console;
pub(crate) use simple_console::*;
mod update_system;
pub(crate) use update_system::*;

use crate::BracketContext;

pub(crate) trait ConsoleFrontEnd: Sync+Send {
    fn cls(&mut self);
    fn print(&mut self, x: usize, y: usize, text: &str);
    fn print_color(&mut self, x: usize, y: usize, text: &str, foreground: Color);

    fn update_mesh(
        &self,
        ctx: &BracketContext,
        meshes: &mut Assets<Mesh>,
    );
}