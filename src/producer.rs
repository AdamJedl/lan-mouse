use std::{error::Error, io};

use futures_core::Stream;

use crate::{client::{ClientHandle, ClientEvent}, event::Event};
use crate::backend::producer;

#[cfg(unix)]
use std::env;

#[cfg(unix)]
enum Backend {
    LayerShell,
    Libei,
    X11,
}

pub fn create() -> Result<Box<dyn EventProducer>, Box<dyn Error>> {
    #[cfg(windows)]
    return Ok(Box::new(producer::windows::WindowsProducer::new()));

    #[cfg(unix)]
    let backend = match env::var("XDG_SESSION_TYPE") {
        Ok(session_type) => match session_type.as_str() {
            "x11" => {
                log::info!("XDG_SESSION_TYPE = x11 -> using X11 event producer");
                Backend::X11
            },
            "wayland" => {
                log::info!("XDG_SESSION_TYPE = wayland -> using wayland event producer");
                match env::var("XDG_CURRENT_DESKTOP") {
                    Ok(desktop) => match desktop.as_str() {
                        "GNOME" => {
                            log::info!("XDG_CURRENT_DESKTOP = GNOME -> using libei backend");
                            Backend::Libei
                        }
                        d => {
                            log::info!("XDG_CURRENT_DESKTOP = {d} -> using layer_shell backend");
                            Backend::LayerShell
                        }
                    }
                    Err(_) => {
                        log::warn!("XDG_CURRENT_DESKTOP not set! Assuming layer_shell support -> using layer_shell backend");
                        Backend::LayerShell
                    }
                }
            }
            _ => panic!("unknown XDG_SESSION_TYPE"),
        },
        Err(_) => panic!("could not detect session type: XDG_SESSION_TYPE environment variable not set!"),
    };

    #[cfg(unix)]
    match backend {
        Backend::X11 => {
            #[cfg(not(feature = "x11"))]
            panic!("feature x11 not enabled");
            #[cfg(feature = "x11")]
            Ok(Box::new(producer::x11::X11Producer::new()))
        }
        Backend::LayerShell => {
            #[cfg(not(feature = "wayland"))]
            panic!("feature wayland not enabled");
            #[cfg(feature = "wayland")]
            Ok(Box::new(producer::wayland::WaylandEventProducer::new()?))
        }
        Backend::Libei => {
            #[cfg(not(feature = "libei"))]
            panic!("feature libei not enabled");
            #[cfg(feature = "libei")]
            Ok(Box::new(producer::libei::LibeiProducer::new()?))
        },
    }
}

pub trait EventProducer: Stream<Item = io::Result<(ClientHandle, Event)>> + Unpin {
    /// notify event producer of configuration changes
    fn notify(&mut self, event: ClientEvent);

    /// release mouse
    fn release(&mut self);
}
