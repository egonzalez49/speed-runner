use speed_runner::{
    configuration::get_configuration, record::record_metrics, speed_test::run_test,
};

fn main() -> anyhow::Result<()> {
    let config = get_configuration()?;

    let test_results = run_test()?;

    record_metrics(test_results, config)?;

    Ok(())
}
