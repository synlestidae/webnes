//
// Author: Patrick Walton
//

#[macro_use]
extern crate lazy_static;

// NB: This must be first to pick up the macro definitions. What a botch.
#[macro_use]
pub mod util;

pub mod apu;
pub mod audio;
#[macro_use]
pub mod cpu;
pub mod resampler;
pub mod disasm;
pub mod gfx;
pub mod input;
pub mod mapper;
pub mod mem;
pub mod ppu;
pub mod rom;
pub mod input_source;
pub mod emulator;
pub mod time;

// C library support
//pub mod speex;

use apu::Apu;
use cpu::Cpu;
use gfx::{Gfx, Scale};
use input::{Input, InputResult};
use mapper::Mapper;
use mem::MemMap;
use ppu::{Oam, Ppu, Vram};
use rom::Rom;
use util::Save;
use std::thread;
use input_source::*;
use emulator::Emulator;

use std::cell::RefCell;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use std::sync::mpsc::channel;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/*fn handle_gamepad_event(key: Keycode, down: bool) -> Option<InputEvent> {
    let event_type = match key {
        Keycode::Left   => EventType::Left,
        Keycode::Down   => EventType::Down,
        Keycode::Up     => EventType::Up,
        Keycode::Right  => EventType::Right,
        Keycode::Z      => EventType::A,
        Keycode::X      => EventType::B,
        Keycode::RShift => EventType::Start,
        Keycode::Return => EventType::Select,
        _               => return None
    };

    Some(InputEvent {
        event_type: event_type,
        active: down
    })
}*/

/// Starts the emulator main loop with a ROM and window scaling. Returns when the user presses ESC.
pub fn start_emulator(rom: Rom, scale: Scale) {
    /*let rom = Box::new(rom);
    println!("Loaded ROM: {}", rom.header);

    let (mut gfx, mut sdl) = Gfx::new(scale);
    let audio_buffer = audio::open();

    let mapper: Box<Mapper+Send> = mapper::create_mapper(rom);
    let mapper = Rc::new(RefCell::new(mapper));
    let ppu = Ppu::new(Vram::new(mapper.clone()), Oam::new());

    let apu = Apu::new(audio_buffer);
    let memmap = MemMap::new(ppu, Input::new(), mapper, apu);

    let mut emulator = Emulator::new(memmap, gfx);

    // TODO: Add a flag to not reset for nestest.log

    loop {
        emulator.step();

        for ev in sdl.event_pump().poll_iter() {
            let (e, active) = match ev {
                Event::KeyDown { keycode: Some(Keycode::S), .. } => (EventType::Save, true),
                Event::KeyDown { keycode: Some(Keycode::L), .. } => (EventType::Load, true),
                Event::KeyDown { keycode: Some(key), .. } => {
                    match handle_gamepad_event(key, true) {
                        Some(ie) => (ie.event_type, true),
                        _ => continue
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    match handle_gamepad_event(key, false) {
                        Some(ie) => (ie.event_type, false),
                        _ => continue
                    }
                },
                Event::Quit { .. } => break,
                _ => continue
            };

            emulator.input(&InputEvent { active: active, event_type: e});
        }

    }

    audio::close();*/
}
