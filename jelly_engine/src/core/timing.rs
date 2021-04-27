use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use super::{system::Write, world::Resource, System, World};

#[derive(Debug)]
pub struct Time {
    delta: Duration,
}

impl Resource for Time {}
impl Default for Time {
    fn default() -> Self {
        Time {
            delta: Duration::from_secs(1),
        }
    }
}

#[derive(Debug)]
pub struct FrameLimiter {
    fps: u32,
    frame_duration: Duration,
}

impl Default for FrameLimiter {
    fn default() -> Self {
        FrameLimiter::new(60)
    }
}

impl FrameLimiter {
    pub fn new(fps: u32) -> Self {
        FrameLimiter {
            fps,
            frame_duration: Duration::from_secs(1) / fps,
        }
    }
}

#[derive(Debug)]
pub struct TimingSystem {
    last_call: Instant,
    limiter: Option<FrameLimiter>,
}

impl Default for TimingSystem {
    fn default() -> Self {
        TimingSystem {
            last_call: Instant::now(),
            limiter: None,
        }
    }
}

impl TimingSystem {
    pub fn with_limiter(mut self, limiter: FrameLimiter) -> Self {
        self.limiter = Some(limiter);
        self
    }
}

impl<'a> System<'a> for TimingSystem {
    type Data = Write<'a, Time>;

    fn init(&mut self, world: &mut World) {
        self.last_call = Instant::now();
    }
    fn run(&mut self, mut data: Self::Data) {
        let mut finish = Instant::now();
        let mut elapsed = finish - self.last_call;

        if let Some(limiter) = &self.limiter {
            if elapsed < limiter.frame_duration {
                sleep(limiter.frame_duration - elapsed);
                finish = Instant::now();
                elapsed += finish - self.last_call;
            }
        }

        data.delta = elapsed;
        self.last_call = finish;

        println!("time: {:?}", data);
    }
    fn dispose(&mut self, world: &mut World) {}
}
