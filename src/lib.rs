use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MaelstromMessageBodyType {
    #[serde(rename = "init")]
    Init,
    #[serde(rename = "init_ok")]
    InitOK,
    #[serde(rename = "echo")]
    Echo,
    #[serde(rename = "echo_ok")]
    EchoOK,
    #[serde(rename = "generate")]
    Generate,
    #[serde(rename = "generate_ok")]
    GenerateOK,
    #[serde(rename = "broadcast")]
    Broadcast,
    #[serde(rename = "broadcast_ok")]
    BroadcastOK,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "read_ok")]
    ReadOK,
    #[serde(rename = "topology")]
    Topology,
    #[serde(rename = "topology_ok")]
    TopologyOK,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaelstromMessageBody {
    #[serde(rename = "type")]
    body_type: MaelstromMessageBodyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    echo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topology: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaelstromMessage {
    src: String,
    dest: String,
    body: MaelstromMessageBody,
}

impl MaelstromMessage {
    pub fn parse(message: &str) -> Self {
        serde_json::from_str::<MaelstromMessage>(&message).unwrap()
    }

    pub fn eval(&mut self, memory: &mut Vec<i32>) {
        // Swap source and destinations
        let new_src = self.dest.clone();
        let new_dest = self.src.clone();

        self.src = new_src;
        self.dest = new_dest;

        match &self.body.body_type {
            MaelstromMessageBodyType::Init => {
                self.body.body_type = MaelstromMessageBodyType::InitOK;
                self.body.in_reply_to = self.body.msg_id;
            }
            MaelstromMessageBodyType::Echo => {
                self.body.body_type = MaelstromMessageBodyType::EchoOK;
                self.body.in_reply_to = self.body.msg_id;
            }
            MaelstromMessageBodyType::Generate => {
                self.body.body_type = MaelstromMessageBodyType::GenerateOK;
                self.body.in_reply_to = self.body.msg_id;
                self.body.id = Option::from(Uuid::new_v4().to_string());
            }
            MaelstromMessageBodyType::Broadcast => {
                self.body.body_type = MaelstromMessageBodyType::BroadcastOK;
                self.body.in_reply_to = self.body.msg_id;
                memory.push(self.body.message.unwrap());
                self.body.message = Option::None;
            }
            MaelstromMessageBodyType::Read => {
                self.body.body_type = MaelstromMessageBodyType::ReadOK;
                self.body.in_reply_to = self.body.msg_id;
                self.body.messages = Option::from(memory.clone());
            }
            MaelstromMessageBodyType::Topology => {
                self.body.body_type = MaelstromMessageBodyType::TopologyOK;
                self.body.in_reply_to = self.body.msg_id;
                self.body.topology = Option::None;
            }
            _ => eprintln!("unkown message body type"),
        }
    }
}

impl fmt::Display for MaelstromMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = serde_json::to_string(self).unwrap();
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_respond_to_init_message() {
        let message = r#"{"src": "c1","dest": "n1","body": {"type": "init","msg_id": 1}}"#;
        let mut maelstrom_message: MaelstromMessage = MaelstromMessage::parse(message);
        maelstrom_message.eval(&mut Vec::new());
        assert_eq!(maelstrom_message.dest, "c1");
        assert_eq!(maelstrom_message.src, "n1");
        assert_eq!(maelstrom_message.body.in_reply_to, Option::from(1));
        assert_eq!(
            maelstrom_message.body.body_type,
            MaelstromMessageBodyType::InitOK
        )
    }
    #[test]
    fn can_respond_to_echo_message() {
        let message = r#"{"src": "c1","dest": "n1","body": {"type": "echo","msg_id": 1,"echo": "Please echo 35"}}"#;
        let mut maelstrom_message: MaelstromMessage = MaelstromMessage::parse(message);
        maelstrom_message.eval(&mut Vec::new());
        assert_eq!(maelstrom_message.dest, "c1");
        assert_eq!(maelstrom_message.src, "n1");
        assert_eq!(maelstrom_message.body.in_reply_to, Option::from(1));
        assert_eq!(
            maelstrom_message.body.body_type,
            MaelstromMessageBodyType::EchoOK
        )
    }
    #[test]
    fn can_respond_to_generate_message() {
        let message = r#"{"src": "c1","dest": "n1","body": {"type": "generate","msg_id": 1}}"#;
        let mut maelstrom_message: MaelstromMessage = MaelstromMessage::parse(message);
        maelstrom_message.eval(&mut Vec::new());
        assert_eq!(maelstrom_message.dest, "c1");
        assert_eq!(maelstrom_message.src, "n1");
        assert_eq!(maelstrom_message.body.in_reply_to, Option::from(1));
        assert_eq!(
            maelstrom_message.body.body_type,
            MaelstromMessageBodyType::GenerateOK
        )
    }
    #[test]
    fn can_respond_to_broadcast_messag() {
        let message = r#"{"src": "c1","dest": "n1","body": {"type": "broadcast","msg_id": 1, "message": -1}}"#;
        let mut maelstrom_message: MaelstromMessage = MaelstromMessage::parse(message);
        maelstrom_message.eval(&mut Vec::new());
        assert_eq!(maelstrom_message.dest, "c1");
        assert_eq!(maelstrom_message.src, "n1");
        assert_eq!(maelstrom_message.body.in_reply_to, Option::from(1));
        assert_eq!(
            maelstrom_message.body.body_type,
            MaelstromMessageBodyType::BroadcastOK
        )
    }
    #[test]
    fn can_respond_to_read_message() {
        let message1 = r#"{"src": "c1","dest": "n1","body": {"type": "broadcast","msg_id": 1, "message": -1}}"#;
        let message2 =
            r#"{"src": "c1","dest": "n1","body": {"type": "broadcast","msg_id": 2, "message": 0}}"#;
        let message3 = r#"{"src": "c1","dest": "n1","body": {"type": "read","msg_id": 3}}"#;
        let mut memory: Vec<i32> = Vec::new();
        let mut maelstrom_message: MaelstromMessage = MaelstromMessage::parse(message1);
        maelstrom_message.eval(&mut memory);
        maelstrom_message = MaelstromMessage::parse(message2);
        maelstrom_message.eval(&mut memory);
        maelstrom_message = MaelstromMessage::parse(message3);
        maelstrom_message.eval(&mut memory);
        assert_eq!(maelstrom_message.dest, "c1");
        assert_eq!(maelstrom_message.src, "n1");
        assert_eq!(maelstrom_message.body.in_reply_to, Option::from(3));
        assert_eq!(
            maelstrom_message.body.messages,
            Option::from(Vec::from([-1, 0]))
        );
        assert_eq!(
            maelstrom_message.body.body_type,
            MaelstromMessageBodyType::ReadOK
        )
    }
    #[test]
    fn can_respond_to_topology_message() {
        let message = r#"{"src": "c1","dest": "n1","body": {"type": "topology","msg_id": 1, "topology": {"n1": ["n2", "n3"]}}}"#;
        let mut maelstrom_message: MaelstromMessage = MaelstromMessage::parse(message);
        maelstrom_message.eval(&mut Vec::new());
        assert_eq!(maelstrom_message.dest, "c1");
        assert_eq!(maelstrom_message.src, "n1");
        assert_eq!(maelstrom_message.body.in_reply_to, Option::from(1));
    }
}
