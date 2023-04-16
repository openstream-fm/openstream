pub mod v1 {
  use std::net::IpAddr;

  use lazy_regex::regex;

  pub fn parse_ip_from_proxy_line(line: &str) -> Option<IpAddr> {
    let re = regex!(
      r#"^PROXY TCP(?P<version>4|6) (?P<src_addr>[0-9a-fA-F\.:]+) (?P<dst_addr>[0-9a-fA-F\.:]+) (?P<src_port>[0-9]+) (?P<dst_port>[0-9]+)$"#
    );

    let caps = re.captures(line.trim()).unwrap();

    let ip = caps.name("src_addr").unwrap().as_str();

    ip.parse().ok()
  }
}

#[cfg(test)]
mod tests {
  use super::v1::*;

  #[test]
  fn test_parse_ip_from_proxy_line_ipv4() {
    let line = "PROXY TCP4 192.168.1.1 192.168.2.2 12345 80";
    let expected = Some("192.168.1.1".parse().unwrap());
    assert_eq!(parse_ip_from_proxy_line(line), expected);
  }

  #[test]
  fn test_parse_ip_from_proxy_line_ipv6() {
    let line = "PROXY TCP6 fa80::1 ff80::2 12345 80";
    let expected = Some("fa80::1".parse().unwrap());
    assert_eq!(parse_ip_from_proxy_line(line), expected);
  }

  #[test]
  fn test_parse_ip_from_proxy_line_invalid() {
    let line = "PROXY TCP4 192.168.1.1:12345 192.168.2.2 12345 80";
    let expected = None;
    assert_eq!(parse_ip_from_proxy_line(line), expected);
  }
}
