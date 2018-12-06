mod debug_objects;
mod follow;
mod show_activity;
mod show_owner;
mod show_route;
mod turn_cycler;

use ezgui::{Color, GfxCtx};
use objects::{Ctx, ID};
use plugins::{Plugin, PluginCtx};

pub struct ViewMode {
    ambient_plugins: Vec<Box<Plugin>>,
}

impl ViewMode {
    pub fn new() -> ViewMode {
        ViewMode {
            ambient_plugins: vec![
                Box::new(follow::FollowState::new()),
                Box::new(debug_objects::DebugObjectsState::new()),
                Box::new(show_activity::ShowActivityState::new()),
                Box::new(show_owner::ShowOwnerState::new()),
                Box::new(show_route::ShowRouteState::new()),
                Box::new(turn_cycler::TurnCyclerState::new()),
            ],
        }
    }
}

impl Plugin for ViewMode {
    fn event(&mut self, mut ctx: PluginCtx) -> bool {
        for p in self.ambient_plugins.iter_mut() {
            p.ambient_event(&mut ctx);
        }
        false
    }

    fn draw(&self, g: &mut GfxCtx, mut ctx: Ctx) {
        for p in &self.ambient_plugins {
            p.new_draw(g, &mut ctx);
        }
    }

    fn color_for(&self, obj: ID, mut ctx: Ctx) -> Option<Color> {
        // First one arbitrarily wins.
        // TODO Maybe none of these actually do this?
        for p in &self.ambient_plugins {
            if let Some(c) = p.new_color_for(obj, &mut ctx) {
                return Some(c);
            }
        }
        None
    }
}
