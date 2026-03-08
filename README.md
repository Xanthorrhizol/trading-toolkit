# trading-toolkit

This crate provides a small set of reusable indicator implementations and core data traits so you can plug in your own candle / OHLCV types without being forced into a specific data model.

## Features

- Generic traits for market data
  - `BaseData`
  - `Stock`
- Core utility types
  - `Time`
  - `ToolkitError`
- Technical indicators
  - Moving Average
    - Simple Moving Average (`SMA`)
    - Exponential Moving Average (`EMA`)
  - MACD
  - Force Index
  - Stochastic
    - Fast Stochastic
    - Slow Stochastic
  - Channel
    - Envelope
    - Bollinger Band
  - Elder Ray

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
trading-toolkit = { git = "https://github.com/Xanthorrhizol/trading-toolkit", branch = "main" }
```

## Design

The crate is built around two traits:

### `BaseData`

A minimal trait for numeric time-series data.

```rust
pub trait BaseData {
    fn value(&self) -> f64;
    fn weight(&self) -> u64;
    fn epoch_time(&self) -> u128;
}
```

Use this when your indicator only needs a value stream and timestamps.

### `Stock`

A richer trait for OHLCV-like market data.

```rust
pub trait Stock {
    fn open_price(&self) -> f64;
    fn high_price(&self) -> f64;
    fn low_price(&self) -> f64;
    fn close_price(&self) -> f64;
    fn tot_exec_amount(&self) -> f64;
    fn tot_exec_volume(&self) -> u64;
    fn epoch_time(&self) -> u128;
}
```

Use this when your indicator depends on candle structure such as high/low/close/volume.

## Quick Start

Define your own candle type and implement `BaseData` and `Stock` for it.

```rust
use trading_toolkit::types::data::Stock;
use trading_toolkit::types::data::BaseData;

#[derive(Debug, Clone)]
struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    amount: f64,
    volume: u64,
    epoch_time: u128,
}

impl BaseData for Candle {
    fn value(&self) -> f64 { self.close }
    fn weight(&self) -> u64 { 1 }
    fn epoch_time(&self) -> u128 { self.epoch_time }
}

impl Stock for Candle {
    fn open_price(&self) -> f64 { self.open }
    fn high_price(&self) -> f64 { self.high }
    fn low_price(&self) -> f64 { self.low }
    fn close_price(&self) -> f64 { self.close }
    fn tot_exec_amount(&self) -> f64 { self.amount }
    fn tot_exec_volume(&self) -> u64 { self.volume }
    fn epoch_time(&self) -> u128 { self.epoch_time }
}
```

Then create indicators from your data.

## Examples

### Moving Average

```rust
use trading_toolkit::indicator::MovingAverage;

// ...

fn main() {
    let data = vec![
        Candle {
            open: 100.0,
            high: 101.0,
            low: 99.0,
            close: 100.0,
            amount: 1.0,
            volume: 1,
            epoch_time: 1772792627050,
        },
        Candle {
            open: 100.0,
            high: 102.0,
            low: 100.0,
            close: 101.0,
            amount: 1.0,
            volume: 1,
            epoch_time: 1772879016479,
        },
        Candle {
            open: 101.0,
            high: 103.0,
            low: 101.0,
            close: 102.0,
            amount: 1.0,
            volume: 1,
            epoch_time: 1772965365613,
        }
    ];

    let sma = MovingAverage::simple(&data);
    let ema = MovingAverage::exponential(&data);

    println!("SMA: {}", sma.inner());
    println!("EMA: {}", ema.inner());
}
```

### MACD

```rust
use trading_toolkit::indicator::MovingAverageConvergenceDivergence;

// ...

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = sample_candles(); // must contain at least 34 items

    let macd = MovingAverageConvergenceDivergence::new(&data)?;
    println!("Fast line: {}", macd.fast());
    println!("Signal line: {}", macd.slow());
    println!("Histogram: {}", macd.macd_histogram());

    Ok(())
}

fn sample_candles() -> Vec<Candle> {
    vec![
        // ...
    ]
}
```

### Force Index

```rust
use trading_toolkit::indicator::ForceIndex;

// ...

fn main() {
    let prev = Candle {
        open: 100.0,
        high: 103.0,
        low: 99.0,
        close: 101.0,
        amount: 100000.0,
        volume: 1000,
        epoch_time: 1,
    };

    let curr = Candle {
        open: 101.0,
        high: 105.0,
        low: 100.0,
        close: 104.0,
        amount: 150000.0,
        volume: 1200,
        epoch_time: 2,
    };

    let fi = ForceIndex::new(&prev, &curr);
    println!("Force Index: {}", fi.inner());
}
```

### Stochastic

```rust
use trading_toolkit::indicator::Stochastic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = sample_candles();

    let fast = Stochastic::fast(&data)?;
    let slow = Stochastic::slow(&data)?;

    println!("Fast stochastic: {}", fast.inner());
    println!("Slow stochastic: {}", slow.inner());

    Ok(())
}

fn sample_candles() -> Vec<Candle> {
    vec![
        // ...
    ]
}
```

### Channel

```rust
use trading_toolkit::indicator::Channel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = sample_candles();

    let envelope = Channel::envelope(&data, 0.05).inner();
    let bollinger = Channel::bollinger(&data, 2.0, true)?.inner();

    println!(
        "Envelope => upper: {}, mid: {}, lower: {}",
        envelope.upper, envelope.mid, envelope.lower
    );

    println!(
        "Bollinger => upper: {}, mid: {}, lower: {}",
        bollinger.upper, bollinger.mid, bollinger.lower
    );

    Ok(())
}

fn sample_candles() -> Vec<Candle> {
    vec![
        // ...
    ]
}
```

### Elder Ray

```rust
use trading_toolkit::indicator::ElderRay;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = sample_candles();
    let elder = ElderRay::new(&data)?;

    println!("Ask force: {}", elder.ask_force());
    println!("Bid force: {}", elder.bid_force());

    Ok(())
}

fn sample_candles() -> Vec<Candle> {
    vec![
        // ...
    ]
}
```

## Time Utility

The crate also includes a `Time` type that wraps milliseconds since UNIX epoch and supports duration helpers.

```rust
use trading_toolkit::types::time::Time;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Time::now()?;
    let one_day = Time::from_days(1);
    let one_hour = Time::from_hours(1);
    let one_minute = Time::from_minutes(1);
    let one_second = Time::from_seconds(1);

    println!("now = {}", now);
    println!("1 day(ms) = {}", one_day.inner());
    println!("1 hour(ms) = {}", one_hour.inner());
    println!("1 minute(ms) = {}", one_minute.inner());
    println!("1 second(ms) = {}", one_second.inner());

    Ok(())
}
```

## Error Handling

Most fallible constructors return `ToolkitError`.

```rust
pub enum ToolkitError {
    EmptyData,
    DataNotEnough,
    InvalidData,
}
```

Typical failure cases:

* empty input data
* not enough input samples
* invalid input type or mixed indicator state

## Current Module Layout

```text
src
├── indicator
│   ├── channel.rs
│   ├── elder_ray.rs
│   ├── force_index.rs
│   ├── macd.rs
│   ├── moving_average.rs
│   ├── stochastic.rs
│   └── mod.rs
├── types
│   ├── data.rs
│   ├── error.rs
│   ├── time.rs
│   └── mod.rs
└── lib.rs
```

## Notes

* Input data is often sorted internally by `epoch_time()`.
* Several indicators accept generic input as long as the required trait is implemented.
* MACD currently requires at least 34 data points.
* This crate is intentionally small and focused on indicator computation rather than exchange connectivity or strategy execution.

## License

MIT
