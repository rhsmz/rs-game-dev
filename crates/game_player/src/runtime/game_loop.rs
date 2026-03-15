use std::time::{Duration, Instant};

// We will import EngineCore from engine_core crate once we add it there.
use engine_core::EngineCore;

pub struct GameLoop {
    engine: EngineCore,
}

impl GameLoop {
    pub fn new(engine: EngineCore) -> Self {
        Self { engine }
    }

    pub fn run(&mut self) {
        let mut last_time = Instant::now();
        let target_frame_time = Duration::from_secs_f32(1.0 / 60.0);

        loop {
            let now = Instant::now();
            let dt = (now - last_time).as_secs_f32();
            last_time = now;

            self.engine.process_input();
            self.engine.update(dt);
            self.engine.render();

            if self.engine.should_quit() {
                break;
            }

            // Sleep for the remaining time of the frame
            let frame_duration = now.elapsed();
            if frame_duration < target_frame_time {
                std::thread::sleep(target_frame_time - frame_duration);
            }
        }
    }
}
