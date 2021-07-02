pub struct Timer {
    starting_time: f32,
    remaining: f32,
    done: bool,
    // on_complete: Option<dyn FnMut() -> ()>,
}

impl Timer {
    pub fn new(time: f32) -> Self {
        Timer {
            starting_time: time,
            remaining: time,
            done: false,
            // on_complete: None,
        }
    }

    // pub fn with_complete(mut self, ) -> Self {
    //     self.on_complete = Some(on_complete);
    //     self
    // }

    pub fn tick(&mut self, dt: f32) {
        self.remaining -= dt;
    }

    pub fn complete(&self) -> bool {
        self.done
    }

    pub fn restart(&mut self) {
        self.remaining = self.starting_time;
    }
}
