extern crate amethyst;

mod breakout;
mod data;
mod systems;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA},
};
use crate::breakout::Breakout;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use amethyst::utils::application_root_dir;

    // display
    let display_path = format!("{}/resources/display_config.ron", application_root_dir());
    let display_config = DisplayConfig::load(display_path);
    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.79, 0.89, 0.96, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()
                    .with_transparency(ColorMask::all(), ALPHA, None)),
        );

    // input
    use amethyst::input::InputBundle;
    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        );

    let mut game = Application::new("./", Breakout, game_data)?;
    game.run();

    Ok(())
}