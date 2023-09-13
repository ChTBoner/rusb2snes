use rusb2snes::SyncClient;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
enum GameStates {
    Logo,        // 0x00
    TitleScreen, // 0x01 - 1
    OptionMode,  // 0x02 - 2
    Menus,       // 0x04 - 4
    LoadArea,    // 0x05 - 5
    LoadGame,    // 0x06
    Saving,      // 0x07
    Playing,     // 0x08
    DoorTransition,
    Unpausing, // 0x10, 0x11, 0x12
    Pausing,   // 0x0C, 0x0D, 0x0E
    Paused,    // 0x0F
    Dead,      // 0x15, 0x17, 0x18, 0x19, 0x1A
    TimerUp,   // 0x23
    GameOver,  // 0x24
    Demos,     // 0x2A
    NewGame,
    CeresEscape,
    CeresElevator,
    Elevator,
    GameTimeEnd,
    RealTimeEnd,
    Credits,
    OpeningSeq,
    Unknown,
    ProgramStarted,
}

const CLIENT_NAME: &str = "TESTS";

// #[test]
// fn list_devices() {
//     let mut usb2snes = SyncClient::connect().unwrap();


//     usb2snes.set_name(CLIENT_NAME.to_string()).unwrap();

//     let devices = usb2snes.list_device().unwrap();

//     usb2snes.attach(&devices[0]).unwrap();
//     let info = usb2snes.info().unwrap();
//     println!("Attached to {} - {}", info.dev_type, info.version);
// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sm_game_state() -> Result<(), Box<dyn Error>>{
        let game_states: HashMap<u8, GameStates> = HashMap::from([
            (0x00, GameStates::Logo),
            (0x01, GameStates::TitleScreen),
            (0x02, GameStates::OptionMode),
            (0x04, GameStates::Menus),
            (0x05, GameStates::LoadArea),
            (0x06, GameStates::LoadGame),
            (0x07, GameStates::Saving),
            (0x08, GameStates::Playing),
            (0x09, GameStates::DoorTransition),
            (0x10, GameStates::DoorTransition),
            (0x0A, GameStates::DoorTransition),
            (0x0B, GameStates::DoorTransition),
            (0x0C, GameStates::Pausing),
            (0x0D, GameStates::Pausing),
            (0x0E, GameStates::Pausing),
            (0x10, GameStates::Unpausing),
            (0x11, GameStates::Unpausing),
            (0x12, GameStates::Unpausing),
            (0x0F, GameStates::Paused),
            (0x13, GameStates::Dead),
            (0x14, GameStates::Dead),
            (0x15, GameStates::Dead),
            (0x16, GameStates::Dead),
            (0x17, GameStates::Dead),
            (0x18, GameStates::Dead),
            (0x19, GameStates::Dead),
            (0x1A, GameStates::Dead),
            (0x1E, GameStates::OpeningSeq),
            (0x1F, GameStates::NewGame),
            (0x20, GameStates::CeresElevator),
            (0x21, GameStates::CeresEscape),
            (0x22, GameStates::CeresEscape),
            (0x23, GameStates::TimerUp),
            (0x24, GameStates::GameOver),
            (0x25, GameStates::GameOver),
            (0x26, GameStates::GameTimeEnd),
            (0x27, GameStates::Credits),
            (0x28, GameStates::Demos),
            (0x29, GameStates::Demos),
            (0x2A, GameStates::Demos),
            (0x2B, GameStates::Demos),
            (0x2C, GameStates::Demos),
        ]);
        
        let mut usb2snes = SyncClient::connect()?;
        usb2snes.set_name(CLIENT_NAME.to_string())?;
        let devices = usb2snes.list_device()?;
        usb2snes.attach(&devices[0])?;
        let state = usb2snes.get_address(0xF50998, 1)?;
        // dbg!(&state);
        // assert_eq!(state[0], 0x04)
        match game_states.get(&state[0]) {
            Some(_) => Ok(()),
            None => panic!(),
        }

    }
}