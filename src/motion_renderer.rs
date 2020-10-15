#[allow(dead_code)]
pub struct MotionRenderer {
    pub width: f64,
    pub height: f64,
    pub fps: f64,
    pub period: f64,
    pub cx: f64,
    pub cy: f64
}


impl MotionRenderer {
    pub fn new(width: f64, height: f64, fps: f64) -> MotionRenderer {
        MotionRenderer {
            width,
            height,
            fps,
            period: 1.0 / fps,
            cx: width / 2.0,
            cy: height / 2.0
        }
    }
}
