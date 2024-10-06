use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ScaleFactor(f64);

impl Default for ScaleFactor {
    fn default() -> Self {
        Self(1.0)
    }
}

impl From<f64> for ScaleFactor {
    fn from(value: f64) -> Self {
        ScaleFactor(value.clamp(0.1, 3.0))
    }
}

impl From<ScaleFactor> for f64 {
    fn from(value: ScaleFactor) -> Self {
        value.0
    }
}

impl ScaleFactor {
    pub fn increment(&mut self) {
        if self.0 < 3.0 {
            self.0 += 0.2;
        }
    }

    pub fn decrement(&mut self) {
        if self.0 >= 0.3 {
            self.0 -= 0.2;
        }
    }
}
