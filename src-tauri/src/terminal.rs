use anyhow::Result;
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub struct Terminal(Arc<Mutex<TerminalInner>>);

impl Terminal {
    pub fn new() -> Self {
        Terminal(Arc::new(Mutex::new(TerminalInner::new())))
    }

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

struct TerminalInner {
    master: Option<Box<dyn MasterPty + Send>>,
    child: Option<Box<dyn Child + Send + Sync>>,
    writer: Option<Box<dyn std::io::Write + Send>>,
}

impl TerminalInner {
    pub fn new() -> Self {
        Self {
            master: None,
            child: None,
            writer: None,
        }
    }

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
            let app_handle = Rc::new(app_handle);

            loop {
                let read_bytes = reader.read(&mut buf).unwrap();

                if read_bytes == 0 {
                    break;
                }

                let bytes = buf[..read_bytes].to_vec();

                app_handle.clone().emit_all("data", bytes).unwrap();
            }
        });

        self.writer.replace(pair.master.take_writer()?);
        self.master.replace(pair.master);

        Ok(())
    }

    pub fn write(&mut self, data: String) -> Result<()> {
        write!(self.writer.as_mut().unwrap(), "{}", data)?;

        Ok(())
    }

    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<()> {
        self.master.as_mut().unwrap().resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        Ok(())
    }

    pub fn dispose(&mut self) -> Result<()> {
        self.child.as_mut().unwrap().kill()?;

        Ok(())
    }
}
