use ggez_video::ffmpeg;

struct MainState {
    video: ggez_video::video::Video,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let video =
            ggez_video::video::Video::from_path(&mut ctx.gfx, "resources/twitch.mp4".to_string())?;

        // ggez_video::video::decode(&mut ctx.gfx, "resources/test.mp4".to_string());

        Ok(MainState { video })
    }
}

impl ggez::event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = ggez::graphics::Canvas::from_frame(
            ctx,
            ggez::graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        self.video.draw(&mut ctx.gfx, &mut canvas)?;

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    ggez_video::init();
    let cb = ggez::ContextBuilder::new("ggez video test", "Bowarc")
        .window_mode(ggez::conf::WindowMode {
            width: 1920.,
            height: 1080.,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Desktop,
            borderless: false,
            min_width: 1.0,
            max_width: 0.0,
            min_height: 1.0,
            max_height: 0.0,
            resizable: false,
            visible: true,
            transparent: false,
            resize_on_scale_factor_change: false,
            logical_size: None,
        })
        .backend(ggez::conf::Backend::default());
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, state)
}
