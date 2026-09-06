//! Dynamic (subprocess) entrypoint for the traefik plugin.
//!
//! Serves this plugin over the orca socket via the typed `Plugin` builder.
//! The plugin is a `[[bin]]`, owns no runtime, and reaches orca only through
//! the socket. Advertises a single `service` backend.
plugin_toolkit::instrument::bootstrap!();
use plugin_toolkit::plugin::Plugin;
use traefik::TraefikBackend;

fn main() -> plugin_toolkit::anyhow::Result<()> {
    Plugin::named("traefik")
        .version(env!("CARGO_PKG_VERSION"))
        .service(TraefikBackend::new("traefik"))
        .serve()
}
