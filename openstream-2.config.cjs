module.exports = {
  apps: [{
    namespace: "s2",
    name: "openstream",
    exec_mode: "fork",
    time: false,
    merge_logs: true,
    // instance_var: "INSTANCE_ID",
    kill_timeout: 60_000,
    script: "./target/release/openstream",
    args: ["start", "-c", "openstream-2.toml"],
    instances: 1,
    env: {
      RUST_LOG_STYLE: "always",
      FORCE_COLOR: 1,
    }
  }]
}