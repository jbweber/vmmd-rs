use std::net::Ipv4Addr;

pub mod libvirt;

pub fn hexify_ipv4_address(v4_address: &str) -> Result<String, String> {
    let v4_ip: Ipv4Addr = match v4_address.parse() {
        Ok(address) => address,
        Err(_) => return Err(format!("{} not valid ipv4 address", v4_address)),
    };

    Ok(hex::encode(v4_ip.octets()))
}

pub fn unhexify_ipv4_address(hex_address: &str) -> Result<String, String> {
    let v4_bytes = match hex::decode(hex_address) {
        Ok(bytes) => bytes,
        Err(_) => return Err(String::from("unable to decode input")),
    };

    if v4_bytes.len() != 4 {
        return Err(format!(
            "{} does not decode to a 4 byte address",
            hex_address
        ));
    }

    let v4_ip = Ipv4Addr::new(v4_bytes[0], v4_bytes[1], v4_bytes[2], v4_bytes[3]);

    Ok(v4_ip.to_string())
}

mod tests {
    // hexify
    #[test]
    fn test_hexify_ipv4_address_happy_path_1() {
        let address = "10.10.10.10";
        let result = String::from("0a0a0a0a");

        assert_eq!(Ok(result), crate::hexify_ipv4_address(address));
    }

    #[test]
    fn test_hexify_ipv4_address_happy_path_2() {
        let address = "192.10.10.11";
        let result = String::from("c00a0a0b");

        assert_eq!(Ok(result), crate::hexify_ipv4_address(address));
    }

    #[test]
    fn test_hexify_ipv4_address_invalid_address_1() {
        let address = "300.10.10.10";

        assert_eq!(
            Err(String::from("300.10.10.10 not valid ipv4 address")),
            crate::hexify_ipv4_address(address)
        );
    }

    #[test]
    fn test_hexify_ipv4_address_invalid_address_2() {
        let address = "this is not an ip";

        assert_eq!(
            Err(String::from("this is not an ip not valid ipv4 address")),
            crate::hexify_ipv4_address(address)
        );
    }

    #[test]
    fn test_hexify_ipv4_address_ipv4_ipv6() {
        let address = "::1";

        assert_eq!(
            Err(String::from("::1 not valid ipv4 address")),
            crate::hexify_ipv4_address(address)
        );
    }

    // unhexify
    #[test]
    fn test_unhexify_ipv4_address_happy_path_1() {
        let address = "0a0a0a0a";
        let result = String::from("10.10.10.10");

        assert_eq!(Ok(result), crate::unhexify_ipv4_address(address));
    }

    #[test]
    fn test_unhexify_ipv4_address_happy_path_2() {
        let address = "c00a0a0b";
        let result = String::from("192.10.10.11");

        assert_eq!(Ok(result), crate::unhexify_ipv4_address(address));
    }

    #[test]
    fn test_unhexify_ipv4_address_invalid_address_1() {
        let address = "not hex value";

        assert_eq!(
            Err(String::from("unable to decode input")),
            crate::unhexify_ipv4_address(address)
        );
    }

    #[test]
    fn test_unhexify_ipv4_address_invalid_address_2() {
        let address = "0a0a0a0a0a";

        assert_eq!(
            Err(String::from(
                "0a0a0a0a0a does not decode to a 4 byte address"
            )),
            crate::unhexify_ipv4_address(address)
        );
    }
}
