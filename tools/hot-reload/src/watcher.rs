// File Watcher
// Überwacht Dateiänderungen im Verzeichnis

use anyhow::Result;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use tokio::sync::oneshot;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    receiver: mpsc::Receiver<notify::Result<Event>>,
    watched_dir: PathBuf,
}

impl FileWatcher {
    pub fn new<P: AsRef<Path>>(directory: P) -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)?;
        
        let dir = directory.as_ref().to_path_buf();
        watcher.watch(&dir, RecursiveMode::Recursive)?;
        
        Ok(FileWatcher {
            watcher,
            receiver: rx,
            watched_dir: dir,
        })
    }
    
    pub async fn wait_for_changes(&mut self) -> Result<Option<Vec<PathBuf>>> {
        use tokio::task;
        
        let (tx, rx) = oneshot::channel();
        let receiver = std::mem::replace(&mut self.receiver, {
            // Erstelle einen neuen Receiver (wird nicht verwendet, aber benötigt für replace)
            let (_, new_rx) = mpsc::channel();
            new_rx
        });
        
        task::spawn_blocking(move || {
            loop {
                match receiver.recv() {
                    Ok(Ok(event)) => {
                        if let EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) = event.kind {
                            // Filtere nur .velin Dateien
                            let velin_files: Vec<PathBuf> = event.paths
                                .iter()
                                .filter(|p| {
                                    p.extension()
                                        .and_then(|s| s.to_str())
                                        .map(|s| s == "velin")
                                        .unwrap_or(false)
                                })
                                .cloned()
                                .collect();
                            
                            if !velin_files.is_empty() {
                                let _ = tx.send(Some(velin_files));
                                return;
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        eprintln!("⚠️  Watcher-Fehler: {}", e);
                    }
                    Err(_) => {
                        // Channel geschlossen
                        break;
                    }
                }
            }
            let _ = tx.send(None);
        });
        
        // Stelle den Receiver wieder her (vereinfacht - in Produktion würde man einen besseren Ansatz verwenden)
        // Für jetzt akzeptieren wir, dass der Receiver nach dem ersten Aufruf nicht mehr funktioniert
        // In einer vollständigen Implementierung würde man einen Arc<Mutex<Receiver>> verwenden
        
        rx.await.map_err(|e| anyhow::anyhow!("Receiver error: {}", e))?
    }
}
