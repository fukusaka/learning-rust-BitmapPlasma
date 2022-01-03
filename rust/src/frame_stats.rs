use std::time::{Duration, SystemTime};

const MAX_FRAME_STATS: usize = 200;
const MAX_PERIOD_MS: Duration = Duration::from_millis(1500);

struct FrameStats {
    render_time: Duration,
    frame_time: Duration,
}

pub struct Stats {
    first_time: Duration,
    last_time: Duration,
    frame_time: Duration,
    frames: Vec<FrameStats>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            last_time: Duration::from_secs(0),
            first_time: Duration::from_secs(0),
            frame_time: Duration::from_secs(0),
            frames: Vec::with_capacity(MAX_FRAME_STATS),
        }
    }

    fn now() -> Duration {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Clock may have gone backwards")
    }

    pub fn stats_start_frame(&mut self) {
        self.frame_time = Self::now();
    }

    pub fn stats_end_frame(&mut self) {
        let now = Self::now();
        let render_time = now - self.frame_time;
        let frame_time = now - self.last_time;
        let diff_time = now - self.first_time;

        if diff_time >= MAX_PERIOD_MS {
            if !self.frames.is_empty() {
                let num_frames = self.frames.len() as u128;
                let min_render = self.frames.iter().map(|x| x.render_time).min().unwrap();
                let max_render = self.frames.iter().map(|x| x.render_time).max().unwrap();
                let avg_render = self.frames.iter().map(|x| x.render_time).sum::<Duration>();
                let min_frame = self.frames.iter().map(|x| x.frame_time).min().unwrap();
                let max_frame = self.frames.iter().map(|x| x.frame_time).max().unwrap();
                let avg_frame = self.frames.iter().map(|x| x.frame_time).sum::<Duration>();

                info!(
                    "frame/s (avg,min,max) = ({:.1},{:.1},{:.1}) render time ms (avg,min,max) = ({:.1},{:.1},{:.1})",
                    1000_000. / (avg_frame.as_micros() as f64 / num_frames as f64),
                    1000_000. /  max_frame.as_micros() as f64,
                    1000_000. /  min_frame.as_micros() as f64,
                    avg_render.as_micros() as f64 / 1000. / num_frames as f64,
                    min_render.as_micros() as f64 / 1000.,
                    max_render.as_micros() as f64 / 1000.
                );
            }

            self.frames.clear();
            self.first_time = now;
        }

        while self.frames.len() > MAX_FRAME_STATS {
            self.frames.remove(0);
        }

        self.frames.push(FrameStats {
            render_time,
            frame_time,
        });
        self.last_time = now;
    }
}
