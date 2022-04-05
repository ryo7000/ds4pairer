const VENDOR: u16 = 0x054c;
const PRODUCT: u16 = 0x05c4;

// https://www.psdevwiki.com/ps4/DS4-USB#Class_Requests
// Set Report 0x14: 13 AC 9E 17 94 05 B0 56 E8 81 38 08 06 51 41 C0 7F 12 AA D9 66 3C CE
const GET_MAC_REPORT_ID: u8 = 0x12;
const SET_MAC_REPORT_ID: u8 = 0x13;

const LINK_KEY: [u8; 16] = [
    0x56, 0xe8, 0x81, 0x38, 0x08, 0x06, 0x51, 0x41, 0xc0, 0x7f, 0x12, 0xaa, 0x39, 0x66, 0x3c, 0xce,
];

fn pair_device(device: &hidapi::HidDevice, mac_bytes: &[u8; 6]) {
    let mut buf = [0u8; 23];
    buf[0] = SET_MAC_REPORT_ID;

    buf[1..7].clone_from_slice(mac_bytes);
    buf[7..].clone_from_slice(&LINK_KEY[..]);

    device.send_feature_report(&buf).unwrap();
}

fn show_paring(device: &hidapi::HidDevice) {
    let mut buf = [0u8; 16];
    buf[0] = GET_MAC_REPORT_ID;
    device.get_feature_report(&mut buf).unwrap();

    println!(
        "device addr: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        buf[6], buf[5], buf[4], buf[3], buf[2], buf[1]
    );
    println!(
        "  host addr: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        buf[15], buf[14], buf[13], buf[12], buf[11], buf[10]
    );
}

fn mac_to_bytes(mac: &str) -> [u8; 6] {
    let mut buf = [0u8; 6];
    let len = buf.len();

    let mut search = &mac[0..];
    for n in (0..len).rev() {
        hex::decode_to_slice(&search[0..2], &mut buf[n..n + 1]).unwrap();

        if n != 0 {
            search = &search[search.find(':').unwrap() + 1..];
        }
    }

    buf
}

fn main() {
    if std::env::args().len() >= 3 {
        println!("usage:\n\t {} [mac]\n", std::env::args().next().unwrap());
        std::process::exit(1);
    }

    let api = hidapi::HidApi::new().unwrap();
    let device = match api.open(VENDOR, PRODUCT) {
        Ok(device) => device,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let bytes = match std::env::args().nth(1) {
        Some(mac) => mac_to_bytes(&mac),
        None => {
            show_paring(&device);
            std::process::exit(1);
        }
    };

    pair_device(&device, &bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macto_bytes() {
        let mac = String::from("11:22:33:44:55:66");
        let bytes = mac_to_bytes(&mac);
        assert_eq!([0x66, 0x55, 0x44, 0x33, 0x22, 0x11], bytes);
    }
}
