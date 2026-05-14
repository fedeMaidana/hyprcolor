// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use smithay_client_toolkit::{output::OutputState, registry::RegistryState};

use wayland_client::{Connection, EventQueue, globals::registry_queue_init};

use crate::app::AppState;

// ─── < Structs > ────────────────────────────────────────────────────

pub struct WaylandState {
    pub registry_state: RegistryState,
    pub output_state: OutputState,
}

// ─── < Public Functions > ────────────────────────────────────────────────────

pub fn init(conn: &Connection) -> Result<(WaylandState, EventQueue<AppState>)> {
    let (globals, event_queue) =
        registry_queue_init::<AppState>(conn).context("registry_queue_init failed")?;

    let qh = event_queue.handle();

    let registry_state = RegistryState::new(&globals);
    let output_state = OutputState::new(&globals, &qh);

    Ok((
        WaylandState {
            registry_state,
            output_state,
        },
        event_queue,
    ))
}
