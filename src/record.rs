use anyhow::Ok;
use lazy_static::lazy_static;
use prometheus::{labels, register_histogram, Histogram};

use crate::{configuration::Settings, speed_test::TestResults};

lazy_static! {
    static ref DL_BANDWIDTH_HISTOGRAM: Histogram =
        register_histogram!("download_bandwidth_mbps", "The download bandiwdth in Mbps").unwrap();
    static ref UL_BANDWIDTH_HISTOGRAM: Histogram =
        register_histogram!("upload_bandwidth_mbps", "The upload bandiwdth in Mbps").unwrap();
    static ref DL_USED_HISTOGRAM: Histogram =
        register_histogram!("download_data_mbps", "Amount of download data used in Mbps").unwrap();
    static ref UL_USED_HISTOGRAM: Histogram =
        register_histogram!("upload_data_mbps", "Amount of upload data used in Mbps").unwrap();
    static ref DL_ELAPSED_HISTOGRAM: Histogram =
        register_histogram!("download_elapsed_milliseconds", "Total time elapsed in ms").unwrap();
    static ref UL_ELAPSED_HISTOGRAM: Histogram =
        register_histogram!("upload_elapsed_milliseconds", "Total time elapsed in ms").unwrap();
}

pub fn record_metrics(test_results: TestResults, config: Settings) -> Result<(), anyhow::Error> {
    let address = config.push_gateway.address();

    DL_BANDWIDTH_HISTOGRAM.observe(test_results.download.bandwidth.as_mbps().into());
    UL_BANDWIDTH_HISTOGRAM.observe(test_results.upload.bandwidth.as_mbps().into());

    DL_USED_HISTOGRAM.observe(test_results.download.used.as_mbps().into());
    UL_USED_HISTOGRAM.observe(test_results.upload.used.as_mbps().into());

    DL_ELAPSED_HISTOGRAM.observe(test_results.download.elapsed.into());
    UL_ELAPSED_HISTOGRAM.observe(test_results.upload.elapsed.into());

    let labels = labels! {"ip".to_owned() => test_results.server.ip};
    let metric_families = prometheus::gather();

    prometheus::push_metrics("speed-runner", labels, &address, metric_families, None)?;

    Ok(())
}
