// ─── < Imports > ────────────────────────────────────────────────────

use smithay_client_toolkit::{
    delegate_output, delegate_registry,
    output::{OutputHandler, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
};

use wayland_client::{Connection, QueueHandle, protocol::wl_output};

use crate::app::AppState;

// ─── < Implementations > ────────────────────────────────────────────────────

// ─── < OutputHandler > ──────────────

impl OutputHandler for AppState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.wayland.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }
}

// ─── < ProvidesRegistryState > ──────────────

impl ProvidesRegistryState for AppState {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.wayland.registry_state
    }

    registry_handlers![OutputState];
}

// ─── < SCTK Delegates > ────────────────────────────────────────────────────

delegate_output!(AppState);
delegate_registry!(AppState);
