use std::io::Read;
use vmmd::libvirt::hooks::qemu;
use vmmd::libvirt::xml::parse_domain_xml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    println!("{:?}", args);

    if args.len() != 5 {
        return Err(format!("expected 5 arguments got {}", args.len()).into());
    }

    let oper = &args[2];
    let sub_oper = &args[3];

    if oper != "started" || sub_oper != "begin" {
        return Ok(());
    }

    let mut domain_xml = String::new();
    std::io::stdin().read_to_string(&mut &mut domain_xml)?;

    let domain = parse_domain_xml(&domain_xml)?;

    qemu(oper, sub_oper, &domain)?;

    Ok(())
}
