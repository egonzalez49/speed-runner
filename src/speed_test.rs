use std::process::Command;

use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TestResults {
    pub download: Metrics,
    pub upload: Metrics,
    pub server: Connection,
}

#[derive(Deserialize)]
pub struct Metrics {
    pub bandwidth: Bytes,
    #[serde(rename = "bytes")]
    pub used: Bytes,
    pub elapsed: u16,
    pub latency: Latency,
}

#[derive(Deserialize)]
pub struct Connection {
    pub ip: String,
}

#[derive(Deserialize)]
pub struct Latency {
    pub iqm: f64,
    pub low: f64,
    pub high: f64,
    pub jitter: f64,
}

#[derive(Deserialize)]
pub struct Bytes(u32);

impl Bytes {
    pub fn as_mbps(&self) -> u32 {
        self.0 / 125_000
    }
}

pub fn run_test() -> Result<TestResults, anyhow::Error> {
    let output = Command::new("speedtest")
        .arg("--format=json")
        .output()
        .expect("failed to execute speedtest");

    let results = serde_json::from_slice::<TestResults>(&output.stdout)
        .context("failed to deserialize test results")?;
    Ok(results)
}
