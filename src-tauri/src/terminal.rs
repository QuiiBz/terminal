use anyhow::{anyhow, Result};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::sync::Mutex;
use tauri::Manager;

#[derive(Default)]
pub struct Terminal(Mutex<TerminalInner>);

impl Terminal {
    pub fn spawn(
        &self,
        app_handle: tauri::AppHandle,
        cols: u16,
        rows: u16,
        shell: &str,
    ) -> Result<()> {
        self.0.lock().unwrap().spawn(app_handle, cols, rows, shell)
    }

    pub fn write(&self, data: String) -> Result<()> {
        self.0.lock().unwrap().write(data)
    }

    pub fn resize(&self, rows: u16, cols: u16) -> Result<()> {
        self.0.lock().unwrap().resize(rows, cols)
    }

    pub fn dispose(&self) -> Result<()> {
        self.0.lock().unwrap().dispose()
    }
}

#[derive(Default)]
struct TerminalInner {
    master: Option<Box<dyn MasterPty + Send>>,
    child: Option<Box<dyn Child + Send + Sync>>,
    writer: Option<Box<dyn std::io::Write + Send>>,
}

impl TerminalInner {
    pub fn spawn(
        &mut self,
        app_handle: tauri::AppHandle,
        cols: u16,
        rows: u16,
        shell: &str,
    ) -> Result<()> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let cmd = CommandBuilder::new(shell);
        let child = pair.slave.spawn_command(cmd)?;

        self.child.replace(child);

        let mut reader = pair.master.try_clone_reader()?;

        std::thread::spawn(move || {
            let mut buf = [0u8; 1024];

            loop {
                // TODO: remove unwrap
                let read_bytes = reader.read(&mut buf).unwrap();

                // Exit the app when we finish reading stdin
                if read_bytes == 0 {
                    app_handle.exit(0);
                    break;
                }

                let bytes = buf[..read_bytes].to_vec();

                // TODO: remove unwrap
                app_handle.emit_all("data", bytes).unwrap();
            }
        });

        self.writer.replace(pair.master.take_writer()?);
        self.master.replace(pair.master);

        Ok(())
    }

    pub fn write(&mut self, data: String) -> Result<()> {
        if let Some(writer) = self.writer.as_mut() {
            writer.write_all(data.as_bytes())?;

            return Ok(());
        }

        Err(anyhow!("Cannot write to terminal without writer"))
    }

    pub fn resize(&self, rows: u16, cols: u16) -> Result<()> {
        if let Some(master) = self.master.as_ref() {
            master.resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })?;

            return Ok(());
        }

        Err(anyhow!("Cannot resize terminal without master"))
    }

    pub fn dispose(&mut self) -> Result<()> {
        if let Some(child) = self.child.as_mut() {
            child.kill()?;

            return Ok(());
        }

        Err(anyhow!("Cannot dispose terminal without child"))
    }
}
