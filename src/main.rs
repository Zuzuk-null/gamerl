use std::cmp::{max, min};

use rltk::{to_cp437, GameState, Point, Rltk, RGB};
use specs::prelude::*;

mod components;
pub use components::*;

mod rect;
pub use rect::*;

mod map;
pub use map::*;

mod player;
pub use player::*;

mod visibility_system;
pub use visibility_system::*;

mod monster_ai_system;
pub use monster_ai_system::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let mut x = 0;
    let mut y = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let (glyph, mut fg) = match tile {
                TileType::Floor => (to_cp437('.'), RGB::from_u8(0, 0x80, 0x80)),
                TileType::Wall => (to_cp437('#'), RGB::from_u8(0, 0xFF, 0)),
            };
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale()
            }
            ctx.set(x, y, fg, rltk::BLACK, glyph);
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }
        draw_map(&self.ecs, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem;
        let mut mob = MonsterAI;
        vis.run_now(&self.ecs);
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

pub fn in_range<T: Ord>((min_bound, max_bound): (T, T), val: T) -> T {
    min(max_bound, max(min_bound, val))
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("sjghs").build()?;
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    let map = Map::new();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(Point::new(player_x, player_y));
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let (glyph, name) = match rng.roll_dice(1, 2) {
            1 => (to_cp437('g'), "???????????? ?????????????? ??????????????".to_string()),
            _ => (to_cp437('o'), "Orc".to_string()),
        };
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster)
            .with(Name {
                name: format!("{} #{}", name, i),
            })
            .build();
    }
    gs.ecs.insert(map);
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player)
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "??????????".to_string(),
        })
        .build();
    rltk::main_loop(context, gs)
}
