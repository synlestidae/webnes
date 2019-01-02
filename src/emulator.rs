use mem::*;
use rom::Rom;
use cpu::{Cpu};
use gfx::*;
use input_source::*;
use input::*;

pub struct Emulator<'a> {
    rom: Box<Rom>,
    //memmap: MemMap,
    cpu: Cpu<MemMap>,
    gfx: Gfx<'a>,
    last_time: f64,
    frames: usize
}

impl<'a> Emulator<'a> {
    pub fn new(mem: MemMap, rom: Box<Rom>, gfx: Gfx<'a>) -> Self {
        Self {
            rom: rom,
            cpu: Cpu::new(mem),
            gfx: gfx,
            last_time: 0.0,
            frames: 0
        }
    }

    pub fn step(&mut self) {
        self.cpu.step();

        let ppu_result = self.cpu.mem.ppu.step(self.cpu.cy);
        if ppu_result.vblank_nmi {
            self.cpu.nmi();
        } else if ppu_result.scanline_irq {
            self.cpu.irq();
        }

        self.cpu.mem.apu.step(self.cpu.cy);

        if ppu_result.new_frame {
            self.gfx.tick();
            self.gfx.composite(&mut *self.cpu.mem.ppu.screen);
            record_fps(&mut self.last_time, &mut self.frames);
            self.cpu.mem.apu.play_channels();

            match self.cpu.mem.input.check_input() {
                InputResult::Continue => {}
                InputResult::Quit => {},
                InputResult::SaveState => {
                    //self.cpu.save(&mut File::create(&Path::new("state.sav")).unwrap());
                    self.gfx.status_line.set("Saved state".to_string());
                }
                InputResult::LoadState => {
                    //self.cpu.load(&mut File::open(&Path::new("state.sav")).unwrap());
                    self.gfx.status_line.set("Loaded state".to_string());
                }
            }
        }
        unimplemented!()
    }

    pub fn input(&mut self, ev: InputEvent) -> InputResult {
        let gamepad = &mut self.cpu.mem.input.gamepad_0;

        match ev.event_type {
            EventType::Right => gamepad.right = ev.active,
            EventType::Down => gamepad.down = ev.active,
            EventType::Left => gamepad.left = ev.active,
            EventType::Up => gamepad.up = ev.active,
            EventType::A => gamepad.a = ev.active,
            EventType::B => gamepad.b = ev.active,
            EventType::Start => gamepad.start = ev.active,
            EventType::Select => gamepad.select = ev.active,
            EventType::Quit => return InputResult::Quit,
            EventType::Save => return InputResult::SaveState,
            EventType::Load => return InputResult::LoadState
        }

        InputResult::Continue
    }
}

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

