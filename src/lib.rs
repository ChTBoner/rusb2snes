use std::net::TcpStream;

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use tungstenite::error::Error;

#[derive(Display, Debug)]
#[allow(dead_code)]
pub enum Command {
    AppVersion,
    Name,
    DeviceList,
    Attach,
    Info,
    Boot,
    Reset,
    Menu,

    List,
    PutFile,
    GetFile,
    Rename,
    Remove,

    GetAddress,
}
#[derive(Display, Debug)]
#[allow(dead_code)]
pub enum Space {
    None,
    SNES,
    CMD,
}

pub struct Infos {
    pub version: String,
    pub dev_type: String,
    pub game: String,
    pub flags: Vec<String>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct USB2SnesQuery {
    Opcode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Space: Option<String>,
    Flags: Vec<String>,
    Operands: Vec<String>,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct USB2SnesResult {
    Results: Vec<String>,
}

#[derive(PartialEq)]
pub enum USB2SnesFileType {
    File = 0,
    Dir = 1,
}
pub struct USB2SnesFileInfo {
    pub name: String,
    pub file_type: USB2SnesFileType,
}

pub struct SyncClient {
    client: WebSocket<MaybeTlsStream<TcpStream>>,
    devel: bool,
}
impl SyncClient {
    pub fn connect() -> Result<SyncClient, Error> {
        let (client, _) = connect("ws://localhost:23074")?;
        Ok(SyncClient {
            client,
            devel: false
        })
    }
    pub fn connect_with_devel() -> Result<SyncClient, Error> {
        let (client, _) = connect("ws://localhost:23074")?;
        Ok(SyncClient {
            client,
            devel: true,
        })
    }
    fn send_command(&mut self, command: Command, args: Vec<String>) -> Result<(), Error> {
        self.send_command_with_space(command, None, args)?;
        Ok(())
    }
    fn send_command_with_space(
        &mut self,
        command: Command,
        space: Option<Space>,
        args: Vec<String>,
    ) -> Result<(), Error> {
        if self.devel {
            println!("Send command : {:?}", command);
        }
        let nspace: Option<String> = space.map(|sp| sp.to_string());
        let query = USB2SnesQuery {
            Opcode: command.to_string(),
            Space: nspace,
            Flags: vec![],
            Operands: args,
        };
        let json = serde_json::to_string_pretty(&query).unwrap();
        if self.devel {
            println!("{}", json);
        }
        let message = Message::text(json);
        self.client.write_message(message)?;
        Ok(())
    }

    fn get_reply(&mut self) -> Result<USB2SnesResult, Error> {
        let reply = self.client.read_message()?;
        let mut textreply: String = String::from("");
        match reply {
            Message::Text(value) => {
                textreply = value;
            }
            _ => {
                println!("Error getting a reply");
            }
        };
        if self.devel {
            println!("Reply:");
            println!("{}", textreply);
        }
        Ok(serde_json::from_str(&textreply).unwrap())
    }
    pub fn set_name(&mut self, name: String) -> Result<(), Error> {
        self.send_command(Command::Name, vec![name])?;
        Ok(())
    }
    pub fn app_version(&mut self) -> Result<String, Error> {
        self.send_command(Command::AppVersion, vec![])?;
        let usbreply = self.get_reply()?;
        Ok(usbreply.Results[0].to_string())
    }
    pub fn list_device(&mut self) -> Result<Vec<String>, Error> {
        self.send_command(Command::DeviceList, vec![])?;
        let usbreply = self.get_reply()?;
        Ok(usbreply.Results)
    }
    pub fn attach(&mut self, device: &String) -> Result<(), Error> {
        self.send_command(Command::Attach, vec![device.to_string()])?;
        Ok(())
    }

    pub fn info(&mut self) -> Result<Infos, Error> {
        self.send_command(Command::Info, vec![])?;
        let usbreply = self.get_reply()?;
        let info: Vec<String> = usbreply.Results;
        Ok(Infos {
            version: info[0].clone(),
            dev_type: info[1].clone(),
            game: info[2].clone(),
            flags: (info[3..].to_vec()),
        })
    }
    pub fn reset(&mut self) -> Result<(), Error> {
        self.send_command(Command::Reset, vec![])?;
        Ok(())
    }
    pub fn menu(&mut self) -> Result<(), Error> {
        self.send_command(Command::Menu, vec![])?;
        Ok(())
    }
    pub fn boot(&mut self, toboot: &str) -> Result<(), Error> {
        self.send_command(Command::Boot, vec![toboot.to_owned()])?;
        Ok(())
    }

    pub fn ls(&mut self, path: &String) -> Result<Vec<USB2SnesFileInfo>, Error> {
        self.send_command(Command::List, vec![path.to_string()])?;
        let usbreply = self.get_reply()?;
        let vec_info = usbreply.Results;
        let mut toret: Vec<USB2SnesFileInfo> = vec![];
        let mut i = 0;
        while i < vec_info.len() {
            let info: USB2SnesFileInfo = USB2SnesFileInfo {
                file_type: if vec_info[i] == "1" {
                    USB2SnesFileType::File
                } else {
                    USB2SnesFileType::Dir
                },
                name: vec_info[i + 1].to_string(),
            };
            toret.push(info);
            i += 2;
        }
        Ok(toret)
    }
    pub fn send_file(&mut self, path: &String, data: Vec<u8>) -> Result<(), Error> {
        self.send_command(
            Command::PutFile,
            vec![path.to_string(), format!("{:x}", data.len())],
        )?;
        let mut start = 0;
        let mut stop = 1024;
        while stop <= data.len() {
            self.client
                .write_message(Message::binary(&data[start..stop]))?;
            start += 1024;
            stop += 1024;
            if stop > data.len() {
                stop = data.len();
            }
        }
        Ok(())
    }
    pub fn get_file(&mut self, path: &str) -> Result<Vec<u8>, Error> {
        self.send_command(Command::GetFile, vec![path.to_owned()])?;
        let string_hex = self.get_reply()?.Results[0].to_string();
        let size = usize::from_str_radix(&string_hex, 16).unwrap();
        let mut data: Vec<u8> = vec![];
        data.reserve(size);
        loop {
            let reply = self.client.read_message().unwrap();
            match reply {
                Message::Binary(msgdata) => {
                    data.extend(&msgdata);
                }
                _ => {
                    println!("Error getting a reply");
                }
            }
            if data.len() == size {
                break;
            }
        }
        Ok(data)
    }
    pub fn get_address(&mut self, address: u32, size: usize) -> Result<Vec<u8>, Error> {
        self.send_command_with_space(
            Command::GetAddress,
            Some(Space::SNES),
            vec![format!("{:x}", address), format!("{:x}", size)],
        )?;
        let mut data: Vec<u8> = vec![];
        data.reserve(size);
        loop {
            let reply = self.client.read_message()?;
            match reply {
                Message::Binary(msgdata) => {
                    data.extend(&msgdata);
                }
                _ => {
                    println!("Error getting a reply");
                }
            }
            if data.len() == size {
                break;
            }
        }
        Ok(data)
    }
    pub fn get_multi_address(
        &mut self,
        addresses: Vec<u32>,
        sizes: Vec<u8>,
    ) -> Result<Vec<u8>, Error> {
        let mut v_arg: Vec<String> = vec![];
        v_arg.reserve(addresses.len() * 2);
        let mut cpt = 0;
        let mut total_size: u8 = 0;
        while cpt < addresses.len() {
            v_arg.push(format!("{:x}", addresses[cpt]));
            v_arg.push(format!("{:x}", sizes[cpt]));
            total_size += sizes[cpt];
            cpt += 1
        }
        self.send_command_with_space(Command::GetAddress, Some(Space::SNES), v_arg)?;
        let mut data: Vec<u8> = vec![];
        data.reserve(total_size as usize);
        loop {
            let reply = self.client.read_message()?;
            match reply {
                Message::Binary(msgdata) => {
                    data.extend(&msgdata);
                }
                _ => {
                    println!("Error getting a reply");
                }
            }
            if data.len() == total_size as usize {
                break;
            }
        }
        Ok(data)
    }
}
