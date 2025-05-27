// use tracing_subscriber::{
//     filter::filter_fn, layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry,
// };

pub fn init_logger() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
