pub fn maxf(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}

pub fn minf(x: f32, y: f32) -> f32 {
    if x > y {
        y
    } else {
        x
    }
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x > max {
        max
    } else if x < min {
        min
    } else {
        x
    }
}