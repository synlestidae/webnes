//
// Author: Patrick Walton
//

#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate sdl2;
extern crate time;

// NB: This must be first to pick up the macro definitions. What a botch.
#[macro_use]
pub mod util;

pub mod apu;
pub mod audio;
#[macro_use]
pub mod cpu;
pub mod disasm;
pub mod gfx;
pub mod input;
pub mod mapper;
pub mod mem;
pub mod ppu;
pub mod rom;
pub mod input_source;
pub mod emulator;

// C library support
pub mod speex;

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

use std::cell::RefCell;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::event::Event::*;
use sdl2::keyboard::Keycode;

use std::sync::mpsc::channel;

fn record_fps(last_time: &mut f64, frames: &mut usize) {
    if cfg!(debug) {
        let now = time::precise_time_s();
        if now >= *last_time + 1f64 {
            println!("{} FPS", *frames);
            *frames = 0;
            *last_time = now;
        } else {
            *frames += 1;
        }
    }
}

fn handle_gamepad_event(key: Keycode, down: bool) -> Option<InputEvent> {
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
}

/// Starts the emulator main loop with a ROM and window scaling. Returns when the user presses ESC.
pub fn start_emulator(rom: Rom, scale: Scale) {
    let rom = Box::new(rom);
    println!("Loaded ROM: {}", rom.header);

    let (mut gfx, mut sdl) = Gfx::new(scale);
    let audio_buffer = audio::open();

    let mapper: Box<Mapper+Send> = mapper::create_mapper(rom);
    let mapper = Rc::new(RefCell::new(mapper));
    let ppu = Ppu::new(Vram::new(mapper.clone()), Oam::new());

    let (send, recv) = channel();

    let input_src = InputSource::new(recv);


    let apu = Apu::new(audio_buffer);
    let memmap = MemMap::new(ppu, Input::new(input_src), mapper, apu);
    let mut cpu = Cpu::new(memmap);

    // TODO: Add a flag to not reset for nestest.log
    cpu.reset();

    let mut last_time = time::precise_time_s();
    let mut frames = 0;

    loop {
        cpu.step();

        let ppu_result = cpu.mem.ppu.step(cpu.cy);
        if ppu_result.vblank_nmi {
            cpu.nmi();
        } else if ppu_result.scanline_irq {
            cpu.irq();
        }

        cpu.mem.apu.step(cpu.cy);

        if ppu_result.new_frame {
            gfx.tick();
            gfx.composite(&mut *cpu.mem.ppu.screen);
            record_fps(&mut last_time, &mut frames);
            cpu.mem.apu.play_channels();

            match cpu.mem.input.check_input() {
                InputResult::Continue => {}
                InputResult::Quit => break,
                InputResult::SaveState => {
                    cpu.save(&mut File::create(&Path::new("state.sav")).unwrap());
                    gfx.status_line.set("Saved state".to_string());
                }
                InputResult::LoadState => {
                    cpu.load(&mut File::open(&Path::new("state.sav")).unwrap());
                    gfx.status_line.set("Loaded state".to_string());
                }
            }
        }

        while let Some(ev) = sdl.event_pump().poll_event() {
            let i = match ev {
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
                Event::Quit { .. } => (EventType::Quit, true),
                _ => continue
            };

            let (event_type, active) = i; 
            send.send(InputEvent { active: active, event_type: event_type });
        }

    }

    audio::close();
}
