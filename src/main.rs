use ggez::{
    conf, event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};
use std::env;
use std::path;

type Point2 = Vec2;

#[derive(Debug)]
enum ActorType {
    Player,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    pos: Point2,
    direction: Direction,
    bbox_size: f32,
    life: f32,
}

const PLAYER_LIFE: f32 = 1.0;
const PLAYER_BBOX: f32 = 32.0;

fn create_player() -> Actor {
    Actor {
        tag: ActorType::Player,
        pos: Point2::ZERO,
        direction: Direction::Left,
        bbox_size: PLAYER_BBOX,
        life: PLAYER_LIFE,
    }
}

struct Assets {
    player_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::from_path(ctx, "/char_1.png")?;

        Ok(Assets { player_image })
    }

    fn actor_image(&self, actor: &Actor) -> &graphics::Image {
        match actor.tag {
            ActorType::Player => &self.player_image,
        }
    }
}

fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: Point2) -> Point2 {
    let x = point.x + screen_width / 2.0;
    let y = screen_height - (point.y + screen_height / 2.0);
    Point2::new(x, y)
}

fn draw_actor(
    assets: &mut Assets,
    canvas: &mut graphics::Canvas,
    actor: &Actor,
    world_coords: (f32, f32),
) {
    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, actor.pos);
    let image = assets.actor_image(actor);
    let drawparams = graphics::DrawParam::new()
        .dest(pos)
        // TODO
        // .rotation(actor.facing)
        .offset(Point2::new(0.5, 0.5));
    canvas.draw(image, drawparams);
}

struct MainState {
    pos_x: f32,
    screen: graphics::ScreenImage,
    assets: Assets,
    player: Actor,
    screen_width: f32,
    screen_height: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let player = create_player();

        let (width, height) = ctx.gfx.drawable_size();
        let screen =
            graphics::ScreenImage::new(ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);

        Ok(MainState {
            pos_x: 0.0,
            screen,
            assets,
            player,
            screen_width: width,
            screen_height: height,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // TODO
        // let mut canvas = graphics::Canvas::from_screen_image(ctx, &mut self.screen, Color::BLACK);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        canvas.draw(&circle, Vec2::new(self.pos_x, 380.0));

        let assets = &mut self.assets;
        let coords = (self.screen_width, self.screen_height);

        {
            let p = &self.player;
            draw_actor(assets, &mut canvas, p, coords);
        }

        let level_dest = Point2::new(10.0, 10.0);
        canvas.draw(
            &graphics::Text::new("hoge"),
            graphics::DrawParam::from(level_dest).color(Color::WHITE),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("hello_ggez", "do7be")
        .window_setup(conf::WindowSetup::default().title("hello_ggez"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
