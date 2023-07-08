use crate::terminal::Terminal;
use serde::{Deserialize, Serialize};
use tauri::{App, Manager};

#[derive(Deserialize, Serialize)]
struct SpawnPayload<'a> {
    cols: u16,
    rows: u16,
    shell: &'a str,
}

#[derive(Deserialize, Serialize)]
struct DataPayload {
    data: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
struct ResizePayload {
    rows: u16,
    cols: u16,
}

pub fn setup(app: &mut App) {
    let app_handle = app.handle();
    app.listen_global("spawn", move |event| {
        let terminal = app_handle.state::<Terminal>();
        let payload = serde_json::from_str::<SpawnPayload>(event.payload().unwrap()).unwrap();

        // TODO: remove unwrap
        terminal
            .spawn(
                app_handle.clone(),
                payload.cols,
                payload.rows,
                payload.shell,
            )
            .unwrap();
    });

    let app_handle = app.handle();
    app.listen_global("write", move |event| {
        let terminal = app_handle.state::<Terminal>();
        let payload = serde_json::from_str::<DataPayload>(event.payload().unwrap()).unwrap();

        // TODO: remove unwrap
        terminal.write(payload.data).unwrap();
    });

    let app_handle = app.handle();
    app.listen_global("resize", move |event| {
        let terminal = app_handle.state::<Terminal>();
        let payload = serde_json::from_str::<ResizePayload>(event.payload().unwrap()).unwrap();

        // TODO: remove unwrap
        terminal.resize(payload.rows, payload.cols).unwrap();
    });

    let app_handle = app.handle();
    app.listen_global("dispose", move |_| {
        let terminal = app_handle.state::<Terminal>();

        // TODO: remove unwrap
        terminal.dispose().unwrap();
    });

    app.listen_global("open", move |event| {
        let payload = serde_json::from_str::<&str>(event.payload().unwrap()).unwrap();

        // TODO: remove unwrap
        open::that(payload).unwrap();
    });
}
