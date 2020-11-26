use noise::*;


#[derive(Clone, Copy, Debug)]
pub struct Gradient {
    pub x_start: f64,
    pub x_stop: f64,
    pub y_start: f64,
    pub y_stop: f64,
    pub z_start: f64,
    pub z_stop: f64,
    pub w_start: f64,
    pub w_stop: f64
}

// Generates a linear gradient for use with noise functions
impl Gradient {
    pub fn new() -> Self {
        Self {
            x_start: 0.0,
            x_stop: 0.0,
            y_start: 0.0,
            y_stop: 0.0,
            z_start: 0.0,
            z_stop: 0.0,
            w_start: 0.0,
            w_stop: 0.0
        }
    }

    pub fn x(self) -> f64 {
        self.x_stop - self.x_start
    }

    pub fn y(self) -> f64 {
        self.y_stop - self.y_start
    }

    pub fn z(self) -> f64 {
        self.z_stop - self.z_start
    }

    pub fn w(self) -> f64 {
        self.w_stop - self.w_start
    }

    pub fn vlen(self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w()
    }

    pub fn set_x_start(self, x_start: f64) -> Self {
        Self {
            x_start,
            ..self
        }
    }

    pub fn set_x_stop(self, x_stop: f64) -> Self {
        Self {
            x_stop,
            ..self
        }
    }

    pub fn set_y_start(self, y_start: f64) -> Self {
        Self {
            y_start,
            ..self
        }
    }

    pub fn set_y_stop(self, y_stop: f64) -> Self {
        Self {
            y_stop,
            ..self
        }
    }

    pub fn set_z_start(self, z_start: f64) -> Self {
        Self {
            z_start,
            ..self
        }
    }

    pub fn set_z_stop(self, z_stop: f64) -> Self {
        Self {
            z_stop,
            ..self
        }
    }
    
    pub fn set_w_start(self, w_start: f64) -> Self {
        Self {
            w_start,
            ..self
        }
    }

    pub fn set_w_stop(self, w_stop: f64) -> Self {
        Self {
            w_stop,
            ..self
        }
    }
}

impl NoiseFn<[f64; 2]> for Gradient {
    fn get(&self, point: [f64; 2]) -> f64 {
        // Subtract from (1) and take dotprod
        let dx = point[0] - self.x_start;
        let dy = point[1] - self.y_start;
        let mut dp = dx * self.x() + dy * self.y();
        dp /= self.vlen();
        // dp=clamp(dp/self.vlen,0.0,1.0);
        // return lerp(dp,1.0,-1.0);
        dp
    }
}

impl NoiseFn<[f64; 3]> for Gradient {
    fn get(&self, point: [f64; 3]) -> f64 {
        let dx = point[0] - self.x_start;
        let dy = point[1] - self.y_start;
        let dz = point[2] - self.z_start;
        let mut dp = dx * self.x() + dy * self.y() + dz * self.z();
        dp /= self.vlen();
        // dp=clamp(dp/self.vlen,0.0,1.0);
        // return lerp(dp,1.0,-1.0);
        dp
    }
}

impl NoiseFn<[f64; 4]> for Gradient {
    fn get(&self, point: [f64; 4]) -> f64 {
        let dx = point[0] - self.x_start;
        let dy = point[1] - self.y_start;
        let dz = point[2] - self.z_start;
        let dw = point[3] - self.w_start;
        let mut dp = dx * self.x() + dy * self.y() + dz * self.z() + dw * self.w();
        dp /= self.vlen();
        // dp=clamp(dp/self.vlen,0.0,1.0);
        // return lerp(dp,1.0,-1.0);
        dp
    }
}
