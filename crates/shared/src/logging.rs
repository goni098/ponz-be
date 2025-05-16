use tracing::{Metadata, level_filters::LevelFilter};
use tracing_subscriber::{
    EnvFilter, Layer, fmt,
    layer::{Filter, SubscriberExt},
    util::SubscriberInitExt,
};

struct InfoOnlyFilter;

impl<S> Filter<S> for InfoOnlyFilter {
    fn enabled(&self, meta: &Metadata<'_>, _: &tracing_subscriber::layer::Context<'_, S>) -> bool {
        meta.level() == &tracing::Level::INFO || meta.level() == &tracing::Level::DEBUG
    }
}

pub fn set_up(directivies: impl IntoIterator<Item = &'static str>) {
    let mut filter = EnvFilter::builder().from_env().unwrap();

    for directive in directivies {
        filter = filter.add_directive(directive.parse().unwrap());
    }

    let error_layer = fmt::Layer::new()
        .with_writer(std::io::stderr)
        .with_filter(LevelFilter::ERROR);

    let info_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_filter(InfoOnlyFilter)
        .with_filter(filter);

    tracing_subscriber::registry()
        .with(error_layer)
        .with(info_layer)
        .init();
}
