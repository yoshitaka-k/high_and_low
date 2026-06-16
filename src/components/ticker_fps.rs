use std::time::{Duration, Instant};

/// ゲーム tick 数と描画 FPS を計測するヘルパー
#[derive(Debug)]
pub struct TickerFps {
    ticker: u64,
    tick_count: u64,
    last_tick_check: Instant,

    fps: u64,
    frame_count: u64,
    last_fps_check: Instant,
}

impl Default for TickerFps {
    fn default() -> Self {
        Self::new()
    }
}

impl TickerFps {
    /// 新しい TickerFps を作成する
    pub fn new() -> Self {
        Self {
            ticker: 0,
            tick_count: 0,
            last_tick_check: Instant::now(),

            fps: 0,
            frame_count: 0,
            last_fps_check: Instant::now(),
        }
    }

    /// 1 tick 進めたときに呼ぶ
    pub fn on_tick(&mut self) {
        self.tick_count += 1;

        if self.last_tick_check.elapsed() >= Duration::from_secs(1) {
            self.ticker = self.tick_count;
            self.tick_count = 0;
            self.last_tick_check = Instant::now();
        }
    }

    /// 1 フレーム描画したときに呼ぶ
    pub fn on_frame(&mut self) {
        self.frame_count += 1;

        if self.last_fps_check.elapsed() >= Duration::from_secs(1) {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.last_fps_check = Instant::now();
        }
    }

    /// Ticker を取得する
    pub fn ticker(&self) -> u64 {
        self.ticker
    }

    /// FPS を取得する
    pub fn fps(&self) -> u64 {
        self.fps
    }

    /// Ticker と FPS を表示する文字列を取得する
    pub fn display(&self) -> String {
        format!("Ticker: {} | FPS: {:}", self.ticker, self.fps as f64)
    }
}
