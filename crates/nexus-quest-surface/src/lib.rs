use anyhow::Result;
use nexus_command_buffer::CommandBuffer;
use nexus_host::{HostAdapter, InputSnapshot};

#[derive(Debug, Default)]
pub struct QuestSurface {
    started: bool,
    frame: u64,
    last_presented: usize,
}

impl QuestSurface {
    pub fn diagnostics(&self) -> String {
        format!(
            "quest_surface started={} frame={} last_presented={}",
            self.started, self.frame, self.last_presented
        )
    }
}

impl HostAdapter for QuestSurface {
    fn start(&mut self) -> Result<()> {
        self.started = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.started = false;
        Ok(())
    }

    fn read_input(&mut self) -> InputSnapshot {
        InputSnapshot {
            frame: self.frame,
            ..InputSnapshot::default()
        }
    }

    fn tick(&mut self, _dt: f32) -> Result<CommandBuffer> {
        self.frame += 1;
        Ok(CommandBuffer::new(self.frame))
    }

    fn present(&mut self, commands: &CommandBuffer) -> Result<()> {
        self.last_presented = commands.len();
        Ok(())
    }
}
