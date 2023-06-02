// vite.config.ts
import { sveltekit } from "file:///home/ramiro/Dev/openstream/front/app/node_modules/@sveltejs/kit/src/exports/vite/index.js";
import { searchForWorkspaceRoot } from "file:///home/ramiro/Dev/openstream/front/app/node_modules/vite/dist/node/index.js";
import { imagetools } from "file:///home/ramiro/Dev/openstream/front/app/node_modules/vite-imagetools/dist/index.js";
var config = (ctx) => {
  return {
    resolve: {
      dedupe: ["@sveltejs/kit", "svelte", "@mdi/js", "http-status-codes", "date-fns", "kleur"]
    },
    server: {
      https: false,
      host: "0.0.0.0",
      port: 3001,
      strictPort: true,
      proxy: {
        "/api": "https://studio.local.openstream.fm"
      },
      fs: {
        allow: [
          searchForWorkspaceRoot(process.cwd()),
          "../share/src"
        ]
      }
    },
    // @ts-ignore
    plugins: [
      sveltekit(),
      // {
      // 	name: "isomorphic-apexchars",
      // 	config: (config, env) => {
      // 		// for(let i = 0; i < 250000; i++) {
      // 		// 	console.log(`target ${i}: ${config.build?.target}`);
      // 		// }
      // 		if (String(config.build?.target).startsWith("node")) {
      // 			return {
      // 				...config,
      // 				alias: {
      // 					resolve: {
      // 						...config.resolve,
      // 						alias: {
      // 							...config.resolve?.alias,
      // 							"apexcharts": "$share/apexcharts.ssr.ts",
      // 						}
      // 					}
      // 				}
      // 			}
      // 		} else {
      // 			return null;
      // 		}
      // 	}
      // },
      imagetools({
        force: true,
        removeMetadata: true
      })
    ]
  };
};
var vite_config_default = config;
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9yYW1pcm8vRGV2L29wZW5zdHJlYW0vZnJvbnQvYXBwXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCIvaG9tZS9yYW1pcm8vRGV2L29wZW5zdHJlYW0vZnJvbnQvYXBwL3ZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9ob21lL3JhbWlyby9EZXYvb3BlbnN0cmVhbS9mcm9udC9hcHAvdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBzdmVsdGVraXQgfSBmcm9tICdAc3ZlbHRlanMva2l0L3ZpdGUnO1xuaW1wb3J0IHR5cGUgeyBVc2VyQ29uZmlnRm4gfSBmcm9tICd2aXRlJztcbmltcG9ydCB7IHNlYXJjaEZvcldvcmtzcGFjZVJvb3QgfSBmcm9tIFwidml0ZVwiO1xuaW1wb3J0IHsgaW1hZ2V0b29scyB9IGZyb20gXCJ2aXRlLWltYWdldG9vbHNcIjtcblxuY29uc3QgY29uZmlnOiBVc2VyQ29uZmlnRm4gPSAoY3R4KSA9PiB7XG5cblx0cmV0dXJuIHtcblx0XHRyZXNvbHZlOiB7XG5cdFx0XHRkZWR1cGU6IFtcIkBzdmVsdGVqcy9raXRcIiwgXCJzdmVsdGVcIiwgXCJAbWRpL2pzXCIsIFwiaHR0cC1zdGF0dXMtY29kZXNcIiwgXCJkYXRlLWZuc1wiLCBcImtsZXVyXCJdLFxuXHRcdH0sXG5cblx0XHRzZXJ2ZXI6IHtcblx0XHRcdGh0dHBzOiBmYWxzZSxcblx0XHRcdGhvc3Q6IFwiMC4wLjAuMFwiLFxuXHRcdFx0cG9ydDogMzAwMSxcblx0XHRcdHN0cmljdFBvcnQ6IHRydWUsXG5cdFx0XHRwcm94eToge1xuXHRcdFx0XHRcIi9hcGlcIjogXCJodHRwczovL3N0dWRpby5sb2NhbC5vcGVuc3RyZWFtLmZtXCIsXG5cdFx0XHR9LFxuXG5cdFx0XHRmczoge1xuXHRcdFx0XHRhbGxvdzogW1xuXHRcdFx0XHRcdHNlYXJjaEZvcldvcmtzcGFjZVJvb3QocHJvY2Vzcy5jd2QoKSksXG5cdFx0XHRcdFx0XCIuLi9zaGFyZS9zcmNcIixcblx0XHRcdFx0XVxuXHRcdFx0fVxuXHRcdH0sXG5cblx0XHQvLyBAdHMtaWdub3JlXG5cdFx0cGx1Z2luczogW1xuXG5cdFx0XHRzdmVsdGVraXQoKSxcblxuXHRcdFx0Ly8ge1xuXHRcdFx0Ly8gXHRuYW1lOiBcImlzb21vcnBoaWMtYXBleGNoYXJzXCIsXG5cdFx0XHQvLyBcdGNvbmZpZzogKGNvbmZpZywgZW52KSA9PiB7XG5cdFx0XHQvLyBcdFx0Ly8gZm9yKGxldCBpID0gMDsgaSA8IDI1MDAwMDsgaSsrKSB7XG5cdFx0XHQvLyBcdFx0Ly8gXHRjb25zb2xlLmxvZyhgdGFyZ2V0ICR7aX06ICR7Y29uZmlnLmJ1aWxkPy50YXJnZXR9YCk7XG5cdFx0XHQvLyBcdFx0Ly8gfVxuXHRcdFx0Ly8gXHRcdGlmIChTdHJpbmcoY29uZmlnLmJ1aWxkPy50YXJnZXQpLnN0YXJ0c1dpdGgoXCJub2RlXCIpKSB7XG5cdFx0XHQvLyBcdFx0XHRyZXR1cm4ge1xuXHRcdFx0Ly8gXHRcdFx0XHQuLi5jb25maWcsXG5cdFx0XHQvLyBcdFx0XHRcdGFsaWFzOiB7XG5cdFx0XHQvLyBcdFx0XHRcdFx0cmVzb2x2ZToge1xuXHRcdFx0Ly8gXHRcdFx0XHRcdFx0Li4uY29uZmlnLnJlc29sdmUsXG5cdFx0XHQvLyBcdFx0XHRcdFx0XHRhbGlhczoge1xuXHRcdFx0Ly8gXHRcdFx0XHRcdFx0XHQuLi5jb25maWcucmVzb2x2ZT8uYWxpYXMsXG5cdFx0XHQvLyBcdFx0XHRcdFx0XHRcdFwiYXBleGNoYXJ0c1wiOiBcIiRzaGFyZS9hcGV4Y2hhcnRzLnNzci50c1wiLFxuXHRcdFx0Ly8gXHRcdFx0XHRcdFx0fVxuXHRcdFx0Ly8gXHRcdFx0XHRcdH1cblx0XHRcdC8vIFx0XHRcdFx0fVxuXHRcdFx0Ly8gXHRcdFx0fVxuXHRcdFx0Ly8gXHRcdH0gZWxzZSB7XG5cdFx0XHQvLyBcdFx0XHRyZXR1cm4gbnVsbDtcblx0XHRcdC8vIFx0XHR9XG5cdFx0XHQvLyBcdH1cblx0XHRcdC8vIH0sXG5cblx0XHRcdGltYWdldG9vbHMoe1xuXHRcdFx0XHRmb3JjZTogdHJ1ZSxcblx0XHRcdFx0cmVtb3ZlTWV0YWRhdGE6IHRydWUsXG5cdFx0XHR9KSxcblxuXHRcdF1cblx0fTtcbn1cblxuZXhwb3J0IGRlZmF1bHQgY29uZmlnO1xuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUFpUyxTQUFTLGlCQUFpQjtBQUUzVCxTQUFTLDhCQUE4QjtBQUN2QyxTQUFTLGtCQUFrQjtBQUUzQixJQUFNLFNBQXVCLENBQUMsUUFBUTtBQUVyQyxTQUFPO0FBQUEsSUFDTixTQUFTO0FBQUEsTUFDUixRQUFRLENBQUMsaUJBQWlCLFVBQVUsV0FBVyxxQkFBcUIsWUFBWSxPQUFPO0FBQUEsSUFDeEY7QUFBQSxJQUVBLFFBQVE7QUFBQSxNQUNQLE9BQU87QUFBQSxNQUNQLE1BQU07QUFBQSxNQUNOLE1BQU07QUFBQSxNQUNOLFlBQVk7QUFBQSxNQUNaLE9BQU87QUFBQSxRQUNOLFFBQVE7QUFBQSxNQUNUO0FBQUEsTUFFQSxJQUFJO0FBQUEsUUFDSCxPQUFPO0FBQUEsVUFDTix1QkFBdUIsUUFBUSxJQUFJLENBQUM7QUFBQSxVQUNwQztBQUFBLFFBQ0Q7QUFBQSxNQUNEO0FBQUEsSUFDRDtBQUFBO0FBQUEsSUFHQSxTQUFTO0FBQUEsTUFFUixVQUFVO0FBQUE7QUFBQTtBQUFBO0FBQUE7QUFBQTtBQUFBO0FBQUE7QUFBQTtBQUFBO0FBQUE7QUFBQTtBQUFBO0FBQUE7QUFBQTtBQUFBO0FBQUE7QUFBQTtBQUFBO0FBQUE7QUFBQTtBQUFBO0FBQUE7QUFBQTtBQUFBO0FBQUEsTUEyQlYsV0FBVztBQUFBLFFBQ1YsT0FBTztBQUFBLFFBQ1AsZ0JBQWdCO0FBQUEsTUFDakIsQ0FBQztBQUFBLElBRUY7QUFBQSxFQUNEO0FBQ0Q7QUFFQSxJQUFPLHNCQUFROyIsCiAgIm5hbWVzIjogW10KfQo=
