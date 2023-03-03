use rltk::{console, Point};
use specs::prelude::*;

use crate::Name;

use super::{Monster, Viewshed};

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, (player_pos, viewshed, monster, name): Self::SystemData) {
        for (viewshed, monster, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("{} вас категорически приветствует", name.name));
            }
        }
    }
}
