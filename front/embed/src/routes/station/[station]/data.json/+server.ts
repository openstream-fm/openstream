import { error, json, type RequestHandler } from "@sveltejs/kit";
import { Client }  from "../../../../../../packages/client/src";
import { env } from "$env/dynamic/private";
const base_url = env.EMBED_API_BASE_URL;
if(base_url == null) throw new Error("EMBED_API_BASE_URL is not set");
const client = new Client(base_url);

export const GET: RequestHandler = async ({ params }) => {
  const station = await client.embed.get_station(null, null, null, params.station!).catch(e => {
    error(Number(e?.status) || 500, String(e?.message))
  })
  return json(station);
}