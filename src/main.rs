use std::io::stdin;
use std::error::Error;

use midir::{MidiInput, Ignore};

use midi_msg::*;

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    println!("Hello, world!");
    // Launch and handle errors
    match read_loop() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}


fn read_loop() -> Result<(), Box<dyn Error>> {

    let mut midi_in = MidiInput::new("midi test read")?;

    // Setup for midi input
    midi_in.ignore(Ignore::None);

    // Select port
    let ports = midi_in.ports();
    let in_port = match ports.len() {
        0 => return Err("No ports found".into()),
        1 => {
            println!("Selecting sole port available| {}", midi_in.port_name(&ports[0]).unwrap());
            &ports[0]
        }
        _ => { // User selection
            let portnames = midi_in.ports().into_iter()
                .map(|p| midi_in.port_name(&p)).collect::<Result<Vec<_>, _>>()?;
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a midi port")
                .items(&portnames)
                .interact()?;
            
            &ports[selection]
        }
    };

    {
        println!("Opening connection");
        let mut ctx = ReceiverContext::new();
        let _conn = midi_in.connect(in_port, "midi test read", move |stamp, message, _| {
            // Decode
            let (msg, _) = MidiMsg::from_midi_with_context(&message, &mut ctx).expect("Decoded midi message");
            
            
            let msg_string = match msg {
                MidiMsg::ChannelVoice {
                    channel,
                    msg
                } => {
                    Some(format!("{:?} {:?}",channel, msg))
                }
                _ => None
            };

            // Print
            println!("{}: {:?} (len = {})\n\t{}", stamp, message, message.len(), msg_string.unwrap_or("No Decode".to_string()));
        }, ())?;


        println!("Reading... (press return to exit)");
        let mut g = String::new();
        stdin().read_line(&mut g)?;

        println!("Closing connection");
    }
    Ok(())
}