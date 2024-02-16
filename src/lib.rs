use core::fmt;
use serde::{Deserialize, Serialize};
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

    pub fn eval(&mut self) {
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
        maelstrom_message.eval();
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
        maelstrom_message.eval();
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
        maelstrom_message.eval();
        assert_eq!(maelstrom_message.dest, "c1");
        assert_eq!(maelstrom_message.src, "n1");
        assert_eq!(maelstrom_message.body.in_reply_to, Option::from(1));
        assert_eq!(
            maelstrom_message.body.body_type,
            MaelstromMessageBodyType::GenerateOK
        )
    }
}
