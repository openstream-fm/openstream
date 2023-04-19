use std::net::IpAddr;

pub fn ip_to_country_code(addr: &IpAddr) -> Option<String> {
  let entry = ip2geo::search(addr)?;
  Some(entry.country)
}

#[test]
fn it_works() {
  let addr = IpAddr::from([133u8, 132u8, 135u8, 169u8]);
  let code = ip_to_country_code(&addr).unwrap();
  eprintln!("code: {code}");
}
