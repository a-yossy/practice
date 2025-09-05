use std::{
    collections::HashMap,
    fs, io,
    net::{AddrParseError, IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    sync::mpsc,
    thread,
    time::Duration,
};

use byteorder::{BigEndian, WriteBytesExt};
use log::{debug, info, warn};
use pnet::{
    packet::{
        Packet,
        icmp::{
            IcmpTypes,
            echo_request::{EchoRequestPacket, MutableEchoRequestPacket},
        },
        ip::IpNextHeaderProtocols::{self},
    },
    transport::{self, TransportChannelType, TransportProtocol::Ipv4, icmp_packet_iter},
    util::checksum,
};

pub fn is_ipaddr_available(target_ip: Ipv4Addr) -> Result<(), failure::Error> {
    let icmp_buf = create_default_icmp_buffer();

    let icmp_packet = EchoRequestPacket::new(&icmp_buf).unwrap();

    let (mut transport_sender, mut transport_receiver) = transport::transport_channel(
        1024,
        TransportChannelType::Layer4(Ipv4(IpNextHeaderProtocols::Icmp)),
    )?;
    transport_sender.send_to(icmp_packet, IpAddr::V4(target_ip))?;

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut iter = icmp_packet_iter(&mut transport_receiver);
        let (packet, _) = iter.next().unwrap();
        if packet.get_icmp_type() == IcmpTypes::EchoReply {
            match sender.send(true) {
                Err(_) => {
                    info!("icmp timeout");
                }
                _ => {
                    return;
                }
            }
        }
    });

    if receiver.recv_timeout(Duration::from_millis(200)).is_ok() {
        let message = format!("ip addr already in use: {}", target_ip);
        warn!("{}", message);
        Err(failure::err_msg(message))
    } else {
        debug!("not received reply within timeout");
        Ok(())
    }
}

pub fn send_dhcp_broadcast_response(soc: &UdpSocket, data: &[u8]) -> Result<(), failure::Error> {
    let destination: SocketAddr = "255.255.255.255:68".parse()?;
    soc.send_to(data, destination)?;
    Ok(())
}

fn create_default_icmp_buffer() -> [u8; 8] {
    let mut buffer = [0u8; 8];
    let mut icmp_packet = MutableEchoRequestPacket::new(&mut buffer).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    let checksum = checksum(icmp_packet.to_immutable().packet(), 16);
    icmp_packet.set_checksum(checksum);
    buffer
}

pub fn u8_to_ipv4addr(buf: &[u8]) -> Option<Ipv4Addr> {
    if buf.len() == 4 {
        Some(Ipv4Addr::new(buf[0], buf[1], buf[2], buf[3]))
    } else {
        None
    }
}

pub fn load_env() -> HashMap<String, String> {
    let contents = fs::read_to_string(".env").expect("Failed to read env file");
    let lines: Vec<_> = contents.split('\n').collect();
    let mut map = HashMap::new();
    for line in lines {
        let elm: Vec<_> = line.split('=').map(str::trim).collect();
        if elm.len() == 2 {
            map.insert(elm[0].to_string(), elm[1].to_string());
        }
    }
    map
}

pub fn obtain_static_addresses(
    env: &HashMap<String, String>,
) -> Result<HashMap<String, Ipv4Addr>, AddrParseError> {
    let network_addr: Ipv4Addr = env
        .get("NETWORK_ADDR")
        .expect("Missing network_addr")
        .parse()?;

    let subnet_mask: Ipv4Addr = env
        .get("SUBNET_MASK")
        .expect("Missing subnet_mask")
        .parse()?;

    let dhcp_server_address = env
        .get("SERVER_IDENTIFIER")
        .expect("Missing server_identifier")
        .parse()?;

    let default_gateway = env
        .get("DEFAULT_GATEWAY")
        .expect("Missing default_gateway")
        .parse()?;

    let dns_addr = env.get("DNS_SERVER").expect("Missing dns_server").parse()?;

    let mut map = HashMap::new();
    map.insert("network_addr".to_string(), network_addr);
    map.insert("subnet_mask".to_string(), subnet_mask);
    map.insert("dhcp_server".to_string(), dhcp_server_address);
    map.insert("default_gateway".to_string(), default_gateway);
    map.insert("dns_addr".to_string(), dns_addr);
    Ok(map)
}

pub fn make_big_endian_vec_from_u32(i: u32) -> Result<Vec<u8>, io::Error> {
    let mut v = Vec::new();
    v.write_u32::<BigEndian>(i)?;
    Ok(v)
}
