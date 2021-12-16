use quick_xml::de::from_str;
use serde::Deserialize;

pub fn parse_domain_xml(domain_xml: &str) -> Result<Domain, Box<dyn std::error::Error>> {
    match from_str(domain_xml) {
        Ok(x) => Ok(x),
        Err(err) => Err(format!("failed to parse domain XML: {}", err).into()),
    }
}

#[derive(Debug, Deserialize)]
pub struct Devices {
    #[serde(rename = "interface")]
    pub interfaces: Option<Vec<Interface>>,
}

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub devices: Option<Devices>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Interface {
    pub mac: InterfaceMAC,
    pub target: InterfaceTarget,
}

#[derive(Debug, Deserialize)]
pub struct InterfaceMAC {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct InterfaceTarget {
    pub dev: String,
}

impl Domain {
    pub fn interface_names(&self) -> Vec<&str> {
        let devices = match &self.devices {
            Some(device) => device,
            None => {
                return vec![];
            }
        };

        let interfaces = match &devices.interfaces {
            Some(interfaces) => interfaces,
            None => {
                return vec![];
            }
        };

        interfaces
            .into_iter()
            .map(|i| i.target.dev.as_str())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    static DOMAIN: &str = r#"
    <domain>
        <name>name</name>
        <devices>
            <interface>
                <mac address="mac_address"/>
                <target dev="target_dev"/>
            </interface>
            <interface>
                <mac address="mac_address"/>
                <target dev="target_dev"/>
            </interface>
        </devices>
    </domain>
    "#;

    #[test]
    fn test_interface_names() {
        let domain = parse_domain_xml(DOMAIN).unwrap();

        let ifnames = domain.interface_names();

        assert_eq!(ifnames.len(), 2);

        assert_eq!(ifnames[0], "target_dev");
        assert_eq!(ifnames[1], "target_dev");
    }

    #[test]
    fn test_parse_domain_xml() {
        let domain = parse_domain_xml(DOMAIN).unwrap();

        assert_eq!(domain.name, "name");
    }

    #[test]
    fn test_parse_domain_xml_error() {
        let parse_error = parse_domain_xml("").expect_err("error expected");

        assert_eq!(
            parse_error.to_string(),
            "failed to parse domain XML: Expecting Start event"
        )
    }

    #[test]
    fn test_domain() {
        let domain: Domain = from_str(DOMAIN).unwrap();

        assert_eq!(domain.name, "name");

        assert!(domain.devices.is_some());

        let devices = domain.devices.unwrap();

        assert!(devices.interfaces.is_some());

        let interfaces = devices.interfaces.unwrap();

        assert_eq!(interfaces.len(), 2);
    }

    static INTERFACE: &str = r#"
    <interface>
        <mac address="mac_address"/>
        <target dev="target_dev"/>
    </interface>
    "#;

    #[test]
    fn test_interface() {
        let iface: Interface = from_str(INTERFACE).unwrap();

        assert_eq!(iface.mac.address, "mac_address");
        assert_eq!(iface.target.dev, "target_dev");
    }
}
