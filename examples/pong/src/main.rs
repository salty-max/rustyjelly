use jelly_engine::engine::Config;

extern crate jelly_engine;

fn main() -> Result<(), String> {
    jelly_engine::engine::start(Config {
        title: String::from("Pong"),
        virtual_width: 384,
        virtual_height: 216,
        screen_width: 1280,
        screen_height: 720,
    })?;

    Ok(())
}
