use wayland_client::protocol::wl_output::WlOutput;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::protocol::wl_registry::{WlRegistry, Event};
use wayland_protocols_wlr::gamma_control::v1::client::zwlr_gamma_control_manager_v1::ZwlrGammaControlManagerV1;
use wayland_protocols_wlr::gamma_control::v1::client::zwlr_gamma_control_v1::ZwlrGammaControlV1;
use wayland_protocols_wlr::gamma_control::v1::client::zwlr_gamma_control_v1::Event as GammaEvent;
use memfd::{MemfdOptions, FileSeal};
use std::os::fd::{IntoRawFd, OwnedFd};
use zerocopy::IntoBytes;
use std::os::fd::FromRawFd;
use std::os::fd::AsFd;
use std::io::{Write, SeekFrom, Seek};
use std::fs::{self, File};
use std::thread;
use std::time::Duration;
struct AppData {
    monitors: Vec<WlOutput>,
    gamma_manager: Option<ZwlrGammaControlManagerV1>,
    gamma_size: u32
}

impl Dispatch<WlOutput, ()> for AppData {
    fn event(
            _state: &mut Self,
            _proxy: &WlOutput,
            _event: <WlOutput as wayland_client::Proxy>::Event,
            _data: &(),
            _conn: &Connection,
            _qhandle: &QueueHandle<Self>,
        ) {
    }
}

impl Dispatch<ZwlrGammaControlManagerV1, ()> for AppData {
    fn event(
            _state: &mut Self,
            _proxy: &ZwlrGammaControlManagerV1,
            _event: <ZwlrGammaControlManagerV1 as wayland_client::Proxy>::Event,
            _data: &(),
            _conn: &Connection,
            _qhandle: &QueueHandle<Self>,
        ) {
    }
}

impl Dispatch<WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        proxy: &WlRegistry,
        event: Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let Event::Global { name, interface, version } = event {
            if interface == "zwlr_gamma_control_manager_v1" {
                let manager = proxy.bind::<ZwlrGammaControlManagerV1, (), AppData>(
                    name, version, qh, ()
                );
                state.gamma_manager = Some(manager);
            }
            else if interface == "wl_output" {
                let monitor = proxy.bind::<WlOutput, (), AppData>(
                    name,version, qh, ()
                );
                state.monitors.push(monitor);
            }
        }
    }
}

impl Dispatch<ZwlrGammaControlV1, ()> for AppData {
    fn event(
            state: &mut Self,
            _proxy: &ZwlrGammaControlV1,
            event: <ZwlrGammaControlV1 as wayland_client::Proxy>::Event,
            _data: &(),
            _conn: &Connection,
            _qhandle: &QueueHandle<Self>,
        ) {
            if let GammaEvent::GammaSize { size } = event {
                state.gamma_size = size;
            }
    }
}

pub fn rgbcol(level: i32, size: u32) -> Vec<u16> {
    let warmth = level as f64 / 100.0;
    let mut rgb: Vec<u16> = Vec::new();
    
    for i in 0..size {
        let v = (i as f64 / (size - 1) as f64 * 65535.0) as u16;
        rgb.push(v);
    }
    for i in 0..size {
        let v = (i as f64 / (size - 1) as f64 * 65535.0 * (1.0 - warmth * 0.45)) as u16;
        rgb.push(v);
    }
    for i in 0..size {
        let v = (i as f64 / (size - 1) as f64 * 65535.0 * (1.0 - warmth * 0.90)) as u16;
        rgb.push(v);
    }
    
    rgb
}

pub fn mem(rgb: &[u16]) -> OwnedFd {
    let opts = MemfdOptions::default().allow_sealing(true);
    let mfd = opts.create("rgb").unwrap();
    let size = rgb.len() * 2;
    mfd.as_file().set_len(size as u64).unwrap();

    let u8_slice: &[u8] = rgb.as_bytes();
    mfd.as_file().write_all(u8_slice).unwrap();
    mfd.as_file().seek(SeekFrom::Start(0)).unwrap();

    mfd.add_seals(&[
        FileSeal::SealShrink,
        FileSeal::SealGrow,
    ]).unwrap();
    mfd.add_seal(FileSeal::SealSeal).unwrap();

    unsafe { OwnedFd::from_raw_fd(mfd.into_raw_fd()) }

}

pub fn level_save(level: i32) {
    let mut file = File::create("/tmp/night-light").unwrap();
    writeln!(file, "{}", level).unwrap();
}

pub fn daemon() {
    let conn = Connection::connect_to_env().unwrap();
    let mut queue = conn.new_event_queue();
    let qh = queue.handle();
    conn.display().get_registry(&qh, ());
    let mut data = AppData {
        monitors: Vec::new(),
        gamma_manager: None,
        gamma_size: 0
    };
    queue.roundtrip(&mut data).unwrap();

    let mut controls: Vec<ZwlrGammaControlV1> = Vec::new(); 
    if let Some(manager) = &data.gamma_manager {
        for monitor in &data.monitors {
            let control = manager.get_gamma_control(monitor, &qh, ());
            controls.push(control);
        }
    }
    queue.roundtrip(&mut data).unwrap();
    
    let rgb = rgbcol(level, data.gamma_size);
    eprintln!("r: {}, g: {}, b: {}", rgb[0], rgb[4096], rgb[8192]);
    let fd = mem(&rgb);
    for control in &controls {
        control.set_gamma(fd.as_fd());
    }
    queue.roundtrip(&mut data).unwrap();
    loop {
        queue.blocking_dispatch(&mut data).unwrap();
    }
}