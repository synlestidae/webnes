use mem::*;
use rom::Rom;
use cpu::Cpu;
use input_source::*;

pub struct Emulator<M: Mem> {
    rom: Box<Rom>,
    memmap: MemMap,
    Cpu: Cpu<M>,
    last_time: f64,
    frames: usize
}

impl<M: Mem> Emulator<M> {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn step(&mut self) {
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
        unimplemented!()
    }

    pub fn input(&mut self, ev: InputEvent) {
        unimplemented!()
    }
}
