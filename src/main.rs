use std::io;
use std::time::Duration;

fn handle_data(data: &[u8]) {
    println!("{:?}", data)
}

fn main() {
    let mut port = serialport::new("/dev/2JCIE-BU", 115_200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    // little endian
    let read_comm = vec![
        // header
        0x52, 0x42,
        // length (payload ~ CRC)
        0x05, 0x00,
        // payload
        0x01, // command - read
        0x22, 0x50, // address - latest data short (0x5022)
        // CRC-16
        0xe2, 0xbb,
    ];

    let mut serial_buf: Vec<u8> = vec![0; 16];
    match port.write(&read_comm) {
        Ok(_) => loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => handle_data(&serial_buf[..t]),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Err(e) => eprintln!("{:?}", e),
    };
}
