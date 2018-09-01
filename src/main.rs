mod world;
extern crate amethyst;

use world::World;
use amethyst::prelude::*;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{DisplayConfig, DrawFlat, Event, Pipeline, PosNormTex,
                         RenderBundle, Stage, VirtualKeyCode};


impl<'a, 'b> State<GameData<'a, 'b>> for World {

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::build("./", World)?
        .build(game_data)?;
    game.run();
    Ok(())
}
