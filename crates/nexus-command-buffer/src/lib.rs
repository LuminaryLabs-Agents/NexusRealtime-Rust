use serde::{Deserialize, Serialize};

pub type EntityId = String;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderDescriptor {
    pub mesh: String,
    pub material: String,
    pub layer: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostEvent {
    pub name: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NexusCommand {
    CreateEntity { id: EntityId, kind: String },
    DestroyEntity { id: EntityId },
    SetTransform { id: EntityId, transform: Transform },
    SetMesh { id: EntityId, mesh: String, material: String },
    SetText { id: EntityId, text: String },
    SpawnPanel { id: EntityId, position: [f32; 3], label: String },
    EmitEvent { name: String, payload: serde_json::Value },
    HostCommand { command: String, data: serde_json::Value },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandBuffer {
    pub frame: u64,
    pub commands: Vec<NexusCommand>,
}

impl CommandBuffer {
    pub fn new(frame: u64) -> Self {
        Self {
            frame,
            commands: Vec::new(),
        }
    }

    pub fn push(&mut self, command: NexusCommand) {
        self.commands.push(command);
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn summary(&self) -> String {
        format!("frame={} commands={}", self.frame, self.commands.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_buffer_summarizes_frame() {
        let mut buffer = CommandBuffer::new(7);
        buffer.push(NexusCommand::CreateEntity {
            id: "panel".to_string(),
            kind: "diagnostic".to_string(),
        });

        assert_eq!(buffer.summary(), "frame=7 commands=1");
    }
}
