use rusb2snes::SyncClient;

fn main() {
    let mut usb2snes = SyncClient::connect().unwrap();


    usb2snes.set_name(String::from("example")).unwrap();

    let devices = usb2snes.list_device().unwrap();

    usb2snes.attach(&devices[0]).unwrap();
    let info = usb2snes.info().unwrap();
    println!("Attached to {} - {}", info.dev_type, info.version);
    let address = usb2snes.get_address(0xF5008B, 2).unwrap();
    for elt in address {
        dbg!(elt);
    }
}