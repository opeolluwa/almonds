// apollo.config.js
module.exports = {
  client: {
    service: {
      name: "orchard",
      url: "http://localhost:5006/orchard",
    },
    includes: ["app/**/*.vue", "app/**/*.ts"],
  },
};
