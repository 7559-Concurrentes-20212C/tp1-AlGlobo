//its called moving stats because it return stats for a moving window of max size history.capacity
pub struct MovingStats {
    pub successful: u32,
    pub failed: u32,
    pub success_rate: f32,
    pub avg_latency: f32,
    pub highest_latency: f32,
    pub lowest_latency: f32,
    pub top_routes: Vec<(String, usize)>,
}
