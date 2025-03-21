use chrono::NaiveDateTime;
use chrono::{Duration, TimeDelta};
use clap::{Command, arg};
use rand::Rng;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::{error::Error, hash, str::FromStr, u32};

fn main() {
    let matches = Command::new("MyApp")
        .about("Generates Random Data for backtester")
        .subcommand(
            Command::new("futures")
                .about("Generate futures data")
                .arg(arg!(--contract_length <LENGTH>).required(true))
                .arg(arg!(--time_period <PERIOD>).required(true))
                .arg(arg!(--date_range <RANGE>).required(true)),
        )
        .subcommand(Command::new("equities").about("Generate equities data"))
        .get_matches();

    match matches.subcommand() {
        Some(("futures", sub_matches)) => {
            let contract_length = sub_matches
                .get_one::<String>("contract_length")
                .expect("required");
            let time_period = sub_matches
                .get_one::<String>("time_period")
                .expect("required");
            let date_range = sub_matches
                .get_one::<String>("date_range")
                .expect("required");

            generate_futures_data(contract_length, time_period, date_range);
        }
        Some(("equities", _)) => {
            todo!("Handle equities data");
        }
        _ => panic!("Invalid subcommand"),
    }
}

fn parse_daterange_string(
    date_range: &str,
) -> Result<(chrono::NaiveDateTime, chrono::NaiveDateTime), &'static str> {
    let parts: Vec<&str> = date_range.split("|").collect();

    if parts.len() != 2 {
        return Err("Invalid Daterange format");
    }

    let start = parts[0];
    let end = parts[1];

    let start_date_time = match chrono::NaiveDateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S") {
        Ok(dt) => dt,
        Err(_) => return Err("Invalid start date format"),
    };

    let end_date_time = match chrono::NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S") {
        Ok(dt) => dt,
        Err(_) => return Err("Invalid end date format"),
    };

    Ok((start_date_time, end_date_time))
}

fn parse_contract_length_string(contract_length: &str) -> u32 {
    let months_string = contract_length.replace("M", "");
    let months = months_string.parse::<u32>().unwrap();

    return months;
}

#[derive(Debug, serde::Deserialize)]
struct OHLCV {
    expiry: NaiveDateTime,
    symbol: String,
    time: NaiveDateTime,
    askopen: f32,
    askhigh: f32,
    asklow: f32,
    askclose: f32,
    asksize: f32,
    bidopen: f32,
    bidhigh: f32,
    bidlow: f32,
    bidclose: f32,
    bidsize: f32,
    close: f32,
    high: f32,
    low: f32,
    open: f32,
    volume: f32,
}

fn generate_futures_data(contract_length: &str, time_period: &str, date_range_string: &str) {
    let (start_date, end_date) =
        parse_daterange_string(date_range_string).expect("Invalid Daterange format");

    let time_period = parse_time_string(time_period).unwrap();

    let contract_length = parse_contract_length_string(contract_length);

    let mut current_date = start_date;

    while current_date < end_date {
        println!("{:?}", current_date);
        generate_futures_contract(
            time_period,
            current_date,
            current_date
                .checked_add_months(chrono::Months::new(contract_length))
                .unwrap(),
            "VX",
        );
        current_date = current_date
            .checked_add_months(chrono::Months::new(1))
            .unwrap();
    }
}

fn generate_futures_contract(
    time_period: TimeDelta,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    symbol: &str,
) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let mut price = 100.0;
    let mut file = File::create("./data.csv")?;

    // Write CSV header
    writeln!(
        file,
        "expiry,symbol,time,askopen,askhigh,asklow,askclose,asksize,bidopen,bidhigh,bidlow,bidclose,bidsize,close,high,low,open,volume"
    )?;
    let expiry_date = end_date;

    let mut current_time = start_date;
    while current_time < end_date {
        // Add some random price movement for realism
        let volatility = 0.01; // 1% price movement
        let price_change = price * volatility * (rng.random::<f32>() - 0.5) * 2.0;
        price += price_change;

        // Generate random variations for different price points
        let open = price;
        let close = price * (1.0 + 0.005 * (rng.random::<f32>() - 0.5));
        let high = f32::max(open, close) * (1.0 + rng.gen_range(0.0..0.01));
        let low = f32::min(open, close) * (1.0 - rng.gen_range(0.0..0.01));

        // Generate bid/ask spread
        let spread = price * 0.001; // 0.1% spread
        let askopen = open + spread / 2.0;
        let askclose = close + spread / 2.0;
        let askhigh = high + spread / 2.0;
        let asklow = low + spread / 2.0;
        let bidopen = open - spread / 2.0;
        let bidclose = close - spread / 2.0;
        let bidhigh = high - spread / 2.0;
        let bidlow = low - spread / 2.0;

        // Random volume and sizes
        let volume = rng.gen_range(100.0..10000.0);
        let asksize = rng.gen_range(10.0..100.0);
        let bidsize = rng.gen_range(10.0..100.0);

        // Create an OHLCV instance
        let ohlcv = OHLCV {
            expiry: expiry_date,
            symbol: symbol.to_string(),
            time: current_time,
            askopen,
            askhigh,
            asklow,
            askclose,
            asksize,
            bidopen,
            bidhigh,
            bidlow,
            bidclose,
            bidsize,
            close,
            high,
            low,
            open,
            volume,
        };

        // Write to CSV
        let csv_string = format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            ohlcv.expiry.format("%Y-%m-%d %H:%M:%S"),
            ohlcv.symbol,
            ohlcv.time.format("%Y-%m-%d %H:%M:%S"),
            ohlcv.askopen,
            ohlcv.askhigh,
            ohlcv.asklow,
            ohlcv.askclose,
            ohlcv.asksize,
            ohlcv.bidopen,
            ohlcv.bidhigh,
            ohlcv.bidlow,
            ohlcv.bidclose,
            ohlcv.bidsize,
            ohlcv.close,
            ohlcv.high,
            ohlcv.low,
            ohlcv.open,
            ohlcv.volume
        );

        writeln!(file, "{}", csv_string)?;

        // Move to the next time period
        current_time += time_period;
    }

    Ok(())
}

#[derive(Debug)]
struct ParseTimeError(String);

impl fmt::Display for ParseTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse time string: {}", self.0)
    }
}

impl Error for ParseTimeError {}

pub fn parse_time_string(time_str: &str) -> Result<Duration, ParseTimeError> {
    // Find where the number ends and the unit begins
    let split_idx = time_str
        .chars()
        .position(|c| !c.is_ascii_digit())
        .ok_or_else(|| ParseTimeError(format!("No unit found in '{}'", time_str)))?;

    // Parse the number
    let amount: i64 = time_str[..split_idx]
        .parse()
        .map_err(|_| ParseTimeError(format!("Invalid number in '{}'", time_str)))?;

    // Get the unit
    let unit = &time_str[split_idx..];

    // Convert to Duration based on unit
    match unit {
        "m" => Ok(Duration::minutes(amount)),
        "H" => Ok(Duration::hours(amount)),
        "D" => Ok(Duration::days(amount)),
        "W" => Ok(Duration::weeks(amount)),
        "M" if unit == "M" => Ok(Duration::days(amount * 30)), // Month (approximated as 30 days)
        "y" => Ok(Duration::days(amount * 365)),               // Year (approximated as 365 days)
        _ => Err(ParseTimeError(format!(
            "Unknown time unit '{}' in '{}'",
            unit, time_str
        ))),
    }
}
