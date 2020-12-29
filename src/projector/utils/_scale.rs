pub fn scale(value: f32, floor: f32, lever: f32, base: f32, ceil: f32) -> f32 {
    f32::min((f32::max(value, floor) - floor) * lever + base, ceil)
}