use jelly_engine::engine::Config;

extern crate jelly_engine;

fn main() -> Result<(), String> {
    jelly_engine::engine::start(Config {
        title: String::from("Pong"),
        virtual_width: 128,
        virtual_height: 128,
        screen_width: 1024,
        screen_height: 1024,
    })?;

    Ok(())
}
