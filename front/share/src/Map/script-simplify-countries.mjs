import src from "./countries.geo.mjs";
import fs from "fs";

const target_file = "countries.simple.geo.js";

const lines = [];

lines.push("export default {")
lines.push(` type: ${JSON.stringify(src.type)},`)
lines.push(` features: [`)
  for(const item of src.features) {
lines.push(`  {`)
lines.push(`   type: ${JSON.stringify(item.type)},`)
lines.push(`   properties: {`)
lines.push(`    iso2: ${JSON.stringify(item.properties.iso_a2)},`)
lines.push(`    name: ${JSON.stringify(item.properties.name)},`)
lines.push(`   },`)
lines.push(`   geometry: ${JSON.stringify(item.geometry)}`)
lines.push(`  },`)
}
lines.push(` ]`)
lines.push("}")

const buf = lines.join("\n");

fs.writeFileSync(target_file, buf);
console.log("written to", target_file);
console.log("done");
