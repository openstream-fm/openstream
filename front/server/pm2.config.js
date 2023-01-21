module.exports = {
  apps: {
    script: "./openstream-front.js",
    args: ["start"],
    name: "openstream-front",
    instances: 12,
    mode: "cluster",
  }
}