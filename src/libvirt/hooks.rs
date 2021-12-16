use futures_util::TryStreamExt;

pub fn qemu(
    oper: &str,
    sub_oper: &str,
    domain: &crate::libvirt::xml::Domain,
) -> Result<(), Box<dyn std::error::Error>> {
    // logger.debug('skipping non-matching op or sub_op', extra={'execution_id': execution_id})
    if oper != "started" || sub_oper != "begin" {
        // TODO log here
        return Ok(());
    }

    // logger.debug('executing matched op and sub_op', extra={'execution_id': execution_id})

    //let name = &domain.name;

    let interface_names: Vec<&str> = domain
        .interface_names()
        .into_iter()
        .filter(|ifname| ifname.starts_with("vmmd-"))
        .collect();

    // logger.info('configuring interfaces', extra={'execution_id': execution_id, 'machine_name': name, 'interfaces': interfaces})

    for &interface_name in interface_names.iter() {
        let split = interface_name.split("-").collect::<Vec<&str>>();

        let addr = match unhexify_ipv4_address(split[1]) {
            Ok(addr) => addr,
            Err(err) => {
                return Err(err);
            }
        };

        match add_route_to_iface(interface_name, addr, 32) {
            Ok(()) => {}
            Err(err) => {
                return Err(err.into());
            }
        }

        match std::fs::write(
            format!("/proc/sys/net/ipv4/conf/{}/proxy_arp", interface_name),
            "1",
        ) {
            Ok(()) => (),
            Err(err) => return Err(err.into()),
        }

        match std::fs::write(
            format!("/proc/sys/net/ipv4/neigh/{}/proxy_delay", interface_name),
            "0",
        ) {
            Ok(()) => (),
            Err(err) => return Err(err.into()),
        }
    }

    Ok(())
}

#[tokio::main]
async fn add_route_to_iface(
    ifname: &str,
    addr: std::net::Ipv4Addr,
    prefix: u8,
) -> Result<(), std::io::Error> {
    let (connection, handle, _) = match rtnetlink::new_connection() {
        Ok((c, h, m)) => (c, h, m),
        Err(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("failed to connect: {}", err),
            ))
        }
    };

    tokio::spawn(connection);

    // find interface index
    let mut links = handle.link().get().match_name(ifname.to_string()).execute();
    let ifidx = match links.try_next().await {
        Ok(Some(link)) => link.header.index,
        Ok(None) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("interface {} not found", ifname),
            ));
        }
        Err(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("failed to find interface {}: {}", ifname, err),
            ));
        }
    };

    // add route
    match handle
        .route()
        .add()
        .v4()
        .destination_prefix(addr, prefix)
        .output_interface(ifidx)
        .scope(rtnetlink::packet::rtnl::RT_SCOPE_LINK)
        .replace()
        .execute()
        .await
    {
        Ok(_) => return Ok(()),
        Err(err) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("failed to add route to device {}: {}", ifname, err),
            ))
        }
    }
}

fn hexify_ipv4_address(ipv4_address: &std::net::Ipv4Addr) -> String {
    hex::encode(ipv4_address.octets())
}

fn unhexify_ipv4_address(
    hex_address: &str,
) -> Result<std::net::Ipv4Addr, Box<dyn std::error::Error>> {
    let v4_bytes = match hex::decode(hex_address) {
        Ok(bytes) => bytes,
        Err(_) => return Err("unable to decode input".into()),
    };

    if v4_bytes.len() != 4 {
        return Err(format!("{} does not decode to a 4 byte address", hex_address).into());
    }

    Ok(std::net::Ipv4Addr::new(
        v4_bytes[0],
        v4_bytes[1],
        v4_bytes[2],
        v4_bytes[3],
    ))
}

mod tests {
    // hexify
    #[test]
    fn test_hexify_ipv4_address_happy_path_1() {
        let address = std::net::Ipv4Addr::new(10, 10, 10, 10);
        let result = String::from("0a0a0a0a");

        assert_eq!(result, super::hexify_ipv4_address(&address));
    }

    #[test]
    fn test_hexify_ipv4_address_happy_path_2() {
        let address = std::net::Ipv4Addr::new(192, 10, 10, 11);
        let result = String::from("c00a0a0b");

        assert_eq!(result, super::hexify_ipv4_address(&address));
    }

    // unhexify
    #[test]
    fn test_unhexify_ipv4_address_happy_path_1() {
        let address = "0a0a0a0a";
        let result = std::net::Ipv4Addr::new(10, 10, 10, 10);

        assert_eq!(result, super::unhexify_ipv4_address(address).unwrap());
    }

    #[test]
    fn test_unhexify_ipv4_address_happy_path_2() {
        let address = "c00a0a0b";
        let result = std::net::Ipv4Addr::new(192, 10, 10, 11);

        assert_eq!(result, super::unhexify_ipv4_address(address).unwrap());
    }

    #[test]
    fn test_unhexify_ipv4_address_invalid_address_1() {
        let address = "not hex value";

        match super::unhexify_ipv4_address(address) {
            Ok(_) => panic!("error expected"),
            Err(err) => assert_eq!("unable to decode input", err.to_string()),
        }
    }

    #[test]
    fn test_unhexify_ipv4_address_invalid_address_2() {
        let address = "0a0a0a0a0a";

        match super::unhexify_ipv4_address(address) {
            Ok(_) => panic!("error expected"),
            Err(err) => assert_eq!(
                "0a0a0a0a0a does not decode to a 4 byte address",
                err.to_string()
            ),
        }
    }
}
