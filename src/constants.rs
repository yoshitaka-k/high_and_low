/// 1 tick の時間（ミリ秒）
pub const TICK_RATE_MILLIS: u64 = 200;

/// フェーズ間の待機 tick 数（200ms × N ≒ 見える間隔）
pub const PHASE_ADVANCE_DELAY_TICKS: u8 = 8;
