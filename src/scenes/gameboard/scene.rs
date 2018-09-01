use ggez;
use ggez_goodies::scene;
use specs;

use super::models::Gameboard;
use super::views::*;
use input;
use scenes::*;
use world::World;

pub struct GameboardScene {
    gameboard: Gameboard,
    gameboard_view: GameboardView,
    abilities_view: AbilitiesView,
    timer_view: TimerView,
    character_portrait_view: PortraitView,
    opponent_portrait_view: PortraitView,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl GameboardScene {
    pub fn new(ctx: &mut ggez::Context, _world: &mut World) -> Self {
        GameboardScene {
            gameboard: Gameboard::new(),
            gameboard_view: GameboardView::new(GameboardViewSettings::new(ctx).unwrap()),
            abilities_view: AbilitiesView::new(AbilitiesViewSettings::new()),
            timer_view: TimerView::new(TimerViewSettings::new()),
            character_portrait_view: PortraitView::new(PortraitViewSettings::new(500.0, 75.0)),
            opponent_portrait_view: PortraitView::new(PortraitViewSettings::new(640.0, 75.0)),
            dispatcher: Self::register_systems(),
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new().build()
    }
}

impl scene::Scene<World, input::InputEvent> for GameboardScene {
    fn update(&mut self, gameworld: &mut World) -> FSceneSwitch {
        self.dispatcher.dispatch(&mut gameworld.specs_world.res);
        if self.gameboard.solved {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, _gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.gameboard_view.draw(ctx, &self.gameboard)?;
        self.abilities_view.draw(ctx, &[])?;
        self.timer_view.draw(ctx, 0)?;
        self.character_portrait_view.draw(ctx)?;
        self.opponent_portrait_view.draw(ctx)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "Game Board"
    }

    fn input(&mut self, _gameworld: &mut World, ev: input::InputEvent, started: bool) {
        use input::{events::InputEffect, Button};
        match (ev, self.gameboard.selected_cell) {
            (InputEffect::Axis(axis, is_positive), _) if !started => {
                self.gameboard.move_selected_cell(axis, is_positive)
            }
            (InputEffect::Button(button, None), Some(ind)) => match button {
                Button::Num1 => self.gameboard.set(ind, 1),
                Button::Num2 => self.gameboard.set(ind, 2),
                Button::Num3 => self.gameboard.set(ind, 3),
                Button::Num4 => self.gameboard.set(ind, 4),
                Button::Num5 => self.gameboard.set(ind, 5),
                Button::Num6 => self.gameboard.set(ind, 6),
                Button::Num7 => self.gameboard.set(ind, 7),
                Button::Num8 => self.gameboard.set(ind, 8),
                Button::Num9 => self.gameboard.set(ind, 9),
                _ => {}
            },
            (InputEffect::Button(Button::Select, Some((x, y))), _) => self.handle_mouse(x, y),
            (_, _) => {}
        }
    }
}

impl GameboardScene {
    fn handle_mouse(&mut self, x: i32, y: i32) {
        let x = x as f32 - self.gameboard_view.settings.position[0];
        let y = y as f32 - self.gameboard_view.settings.position[1];
        if x >= 0.0
            && x < self.gameboard_view.settings.size
            && y >= 0.0
            && y < self.gameboard_view.settings.size
        {
            let cell_x = (x / self.gameboard_view.settings.size * 9.0) as usize;
            let cell_y = (y / self.gameboard_view.settings.size * 9.0) as usize;
            self.gameboard.selected_cell = Some([cell_x, cell_y]);
        }
    }
}
