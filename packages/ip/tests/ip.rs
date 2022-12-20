#[tokio::test]
async fn get_ip_v4() {
  ip::get_ip_v4().await.unwrap();
}

#[tokio::test]
async fn get_ip_v4_ssl() {
  ip::get_ip_v4_ssl().await.unwrap();
}
