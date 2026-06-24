use std::collections::HashSet;

use anyhow::{Context, Result};
use nexus_command_buffer::{CommandBuffer, NexusCommand, Transform};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(default)]
    pub children: Vec<SequenceNode>,
}

pub struct SequenceRuntime {
    root: SequenceNode,
    executed_once: HashSet<String>,
    frame: u64,
}

impl SequenceRuntime {
    pub fn from_json(source: &str) -> Result<Self> {
        let root: SequenceNode = serde_json::from_str(source).context("failed to parse sequence JSON")?;
        Ok(Self { root, executed_once: HashSet::new(), frame: 0 })
    }

    pub fn tick(&mut self, dt: f32) -> CommandBuffer {
        self.frame += 1;
        let mut buffer = CommandBuffer::new(self.frame);
        let root = self.root.clone();
        self.visit(&root, dt, &mut buffer);
        buffer
    }

    fn visit(&mut self, node: &SequenceNode, dt: f32, buffer: &mut CommandBuffer) {
        match node.node_type.as_str() {
            "flow" | "sequence" | "root" => {
                for child in &node.children { self.visit(child, dt, buffer); }
            }
            "host-command" => self.emit_host_command(node, buffer),
            _ => {
                for child in &node.children { self.visit(child, dt, buffer); }
            }
        }
    }

    fn emit_host_command(&mut self, node: &SequenceNode, buffer: &mut CommandBuffer) {
        if !self.executed_once.insert(node.id.clone()) { return; }
        let command = node.command.clone().unwrap_or_else(|| "host_command".to_string());
        let data = node.data.clone().unwrap_or(Value::Null);
        if command == "spawn_panel" {
            let id = field_string(&data, "id").unwrap_or_else(|| node.id.clone());
            let label = field_string(&data, "label").unwrap_or_else(|| "NexusRealtime Rust Host".to_string());
            let position = field_vec3(&data, "position").unwrap_or([0.0, 1.4, -2.0]);
            buffer.push(NexusCommand::CreateEntity { id: id.clone(), kind: "diagnostic_panel".to_string() });
            buffer.push(NexusCommand::SetTransform { id: id.clone(), transform: Transform { position, ..Transform::default() } });
            buffer.push(NexusCommand::SpawnPanel { id: id.clone(), position, label: label.clone() });
            buffer.push(NexusCommand::SetText { id, text: label });
        } else {
            buffer.push(NexusCommand::HostCommand { command, data });
        }
    }
}

fn field_string(value: &Value, key: &str) -> Option<String> {
    value.get(key)?.as_str().map(ToString::to_string)
}

fn field_vec3(value: &Value, key: &str) -> Option<[f32; 3]> {
    let array = value.get(key)?.as_array()?;
    if array.len() != 3 { return None; }
    Some([array[0].as_f64()? as f32, array[1].as_f64()? as f32, array[2].as_f64()? as f32])
}
