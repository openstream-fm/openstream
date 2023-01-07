module.exports = {
  apps: [{
    instances: 12,
    name: "openstream",
    exec_mode: "fork",
    time: false,
    merge_logs: true,
    instance_var: "INSTANCE_ID",
    kill_timeout: 60_000,
    script: "./target/release/openstream",
    args: ["start"],
    env: {
      RUST_LOG: "warn",
      FORCE_COLOR: 1,
    }
  }]
}