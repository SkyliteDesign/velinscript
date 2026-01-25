pub struct MathStdlib;

impl MathStdlib {
    pub fn generate_clamp_code(value: &str, min: &str, max: &str) -> String {
        format!("{}.clamp({}, {})", value, min, max)
    }

    pub fn generate_lerp_code(a: &str, b: &str, t: &str) -> String {
        format!("{} + ({} - {}) * {}", a, b, a, t)
    }

    pub fn generate_round_to_code(value: &str, decimals: &str) -> String {
        format!(
            "{{
                let factor = 10f64.powi({} as i32);
                ({} * factor).round() / factor
            }}",
            decimals, value
        )
    }

    pub fn generate_random_range_code(min: &str, max: &str) -> String {
        format!(
            "{{
                use rand::Rng;
                rand::thread_rng().gen_range({}..{})
            }}",
            min, max
        )
    }

    pub fn generate_min_code(a: &str, b: &str) -> String {
        format!("{}.min({})", a, b)
    }

    pub fn generate_max_code(a: &str, b: &str) -> String {
        format!("{}.max({})", a, b)
    }

    pub fn generate_abs_code(value: &str) -> String {
        format!("{}.abs()", value)
    }

    pub fn generate_floor_code(value: &str) -> String {
        format!("{}.floor()", value)
    }

    pub fn generate_ceil_code(value: &str) -> String {
        format!("{}.ceil()", value)
    }
}
