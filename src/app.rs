// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use calloop::{
    EventLoop,
    timer::{TimeoutAction, Timer},
};

use calloop_wayland_source::WaylandSource;

use wayland_client::Connection;

use crate::{
    config::Config,
    export::Exporters,
    palette::extract_palette,
    wallpaper::{WallpaperFingerprint, current_wallpaper_path},
    wayland,
};

// ─── < Structs > ────────────────────────────────────────────────────

pub struct AppState {
    pub wayland: wayland::WaylandState,
    pub config: Config,
    pub exporters: Exporters,
    pub last_wallpaper: Option<WallpaperFingerprint>,
    pub should_close: bool,
}

pub struct App;

// ─── < Implementatios > ────────────────────────────────────────────────────

// ── < App > ──────────────────

impl App {
    pub fn run() -> Result<()> {
        let config = Config::load();

        let conn = Connection::connect_to_env().context("no se pudo conectar a Wayland")?;
        let (wayland, event_queue) = wayland::init(&conn)?;

        let exporters = Exporters::new(config.output_dir.clone());

        let mut app = AppState {
            wayland,
            config,
            exporters,
            last_wallpaper: None,
            should_close: false,
        };

        let mut event_loop: EventLoop<AppState> =
            EventLoop::try_new().context("no se pudo crear calloop EventLoop")?;

        let loop_handle = event_loop.handle();

        WaylandSource::new(conn, event_queue)
            .insert(loop_handle.clone())
            .map_err(|err| anyhow::anyhow!("WaylandSource insert failed: {err:?}"))?;

        let timer = Timer::from_duration(app.config.poll_interval);

        loop_handle
            .insert_source(timer, |_event, _meta, app| {
                if let Err(err) = app.refresh_colors() {
                    log::warn!("no se pudieron refrescar colores: {err:#}");
                }

                TimeoutAction::ToDuration(app.config.poll_interval)
            })
            .map_err(|err| anyhow::anyhow!("timer insert failed: {err:?}"))?;

        app.refresh_colors()?;

        while !app.should_close {
            event_loop
                .dispatch(None, &mut app)
                .context("event_loop dispatch")?;
        }

        Ok(())
    }
}

// ── < AppState > ──────────────────

impl AppState {
    fn refresh_colors(&mut self) -> Result<()> {
        let Some(wallpaper) = current_wallpaper_path() else {
            log::debug!("no se detectó wallpaper actual");
            return Ok(());
        };

        let fingerprint = WallpaperFingerprint::from_path(&wallpaper)?;

        if self.last_wallpaper.as_ref() == Some(&fingerprint) {
            return Ok(());
        }

        log::info!("wallpaper changed: {}", wallpaper.display());

        let palette = extract_palette(&wallpaper)?;
        self.exporters.export_all(&palette)?;

        self.last_wallpaper = Some(fingerprint);

        Ok(())
    }
}
