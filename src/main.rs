use vmmd::libvirt::xml::parse_domain_xml;

fn main() {
    let devel05 = match std::fs::read_to_string("devel-05.xml") {
        Ok(x) => x,
        Err(_) => return,
    };

    let domain = parse_domain_xml(&devel05).unwrap();

    println!("{:?}", domain);
}
