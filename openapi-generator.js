import { fileURLToPath } from "url";
import path from "path";
import fs from "fs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const basedir = `${__dirname}/defs/api`;

const methods = ["GET", "POST", "PUT", "PATCH", "DELETE"];

const map_path = (dir) => {
  return `/${dir.replaceAll("[", "{").replaceAll("]", "}")}`;
};

const get_path_params = (dir) => {
  return [...dir.matchAll(/\{(.+?)\}/g)];
}

const securitySchemes = {
  "API Key": {
    type: "apiKey",
    in: "header",
    name: "x-access-token",
  }
}

const security = [
  { "API Key": [] }
]

const paths = {};

const process_dir = (target) => {
  const full_target = `${basedir}${target == null ? "" : `/${target}`}`;
  const filenames = fs.readdirSync(full_target).sort();

  for (const name of filenames) {
    if (methods.includes(name)) {
      const method = name;
      const path = map_path(target);

      const output_path = `${full_target}/${name}/Output.schema.json`;
      const output = JSON.parse(fs.readFileSync(output_path));

      const payload_path = `${full_target}/${name}/Payload.schema.json`;
      let payload = null;
      if (fs.existsSync(payload_path)) payload = JSON.parse(fs.readFileSync(payload_path));

      const query_path = `${full_target}/${name}/Query.schema.json`;
      const query_params = [];
      if(fs.existsSync(query_path)) {
        const schema = JSON.parse(fs.readFileSync(query_path));
        for(const key in schema.properties) {
          query_params.push({
            name: key,
            in: "query",
            required: !!schema.required?.includes(key),
            style: "deepObject",
            explode: true,
            allowReserved: true,
            schema: schema.properties[key],
          })
        }
      }

      const path_params = get_path_params(path).map(name => {
        return {
          name: name[1],
          in: "path",
          required: true,
          schema: {
            type: "string",
          }
        }
      });

      paths[path] ??= {};

      paths[path][method.toLowerCase()] = {
        
        parameters: [
          ...path_params,
          ...query_params,
        ],

        requestBody:
          payload == null
            ? undefined
            : {
                required: true,
                content: {
                  "application/json": {
                    schema: payload,
                  },
                },
              },

        responses: {
          200: {
            content: {
              "application/json": {
                schema: output,
              },
            },
          },
        },
      };
    }

    for (const name of filenames) {
      if (methods.includes(name)) continue;
      if (fs.statSync(`${full_target}/${name}`).isDirectory()) {
        process_dir(`${target == null ? "" : `${target}/`}${name}`);
      }
    }
  }
};

process_dir(null)

  // TODO: don't hardcode this
  paths["/station-pictures"].post.requestBody = {
    required: true,
    content: {
      "image/png": {
        schema: {
          type: "string",
          format: "binary",
        }
      },
      "image/jpeg": {
        schema: {
          type: "string",
          format: "binary",
        }
      },
      "image/webp": {
        schema: {
          type: "string",
          format: "binary",
        }
      }
    }
  };

const document = {
  
  openapi: "3.0.0",

  info: {
    title: "Openstream Media Server API",
  
    termsOfService: "https://openstream.fm/api-terms/",
    
    license: {
      name: "Apache 2.0",
      url: "https://www.apache.org/licenses/LICENSE-2.0.html"
    },
  
    version: "0.1.0",
  },

  servers: [{
    url: "https://api.openstream.fm",
    description: "Openstream production servers",
  }],

  security,

  paths,

  components: {
    securitySchemes,  
  }
}

fs.writeFileSync(`${__dirname}/openapi.json`, JSON.stringify(document, null, 2));