module.exports = {
  apps: [
    {
      name: "mailer-static",
      exec_mode: "fork",
      script: "static-web-server",
      args: ["-w", "config.toml"],
      instances: 1,
      kill_timeout: 10_000,
      env: {
        RUST_LOG_STYLE: "always",
        FORCE_COLOR: "1",
      },
    },
  ],
};
