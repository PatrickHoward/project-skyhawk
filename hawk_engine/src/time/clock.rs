pub(crate) struct Clock {
    dt: f64,
    last_tick: u64,
}

impl Clock {
    pub fn new(first_tick: u64) -> Self {
        let dt = 0.0f64;
        let last_tick = first_tick;

        Clock { dt, last_tick }
    }

    pub fn tick(&mut self, tick: u64, freq: u64) {
        self.dt = (tick - self.last_tick) as f64 / freq as f64;
        self.last_tick = tick;
    }

    pub fn throttle(&self, target_fps: i32) {
        target_fps;
    }

    pub fn dt(&self) -> f64 {
        self.dt
    }
}
