module.exports = {
  apps: {
    script: "./openstream-front.js",
    args: ["start"],
    interpreter_args: ["--no-warnings", "--experimental-specifier-resolution=node"],
    name: "openstream-front",
    instances: 2,
    mode: "cluster",
  }
}