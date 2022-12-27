import { error } from "@sveltejs/kit"

export const load = () => {
  throw error(404, { status: 404, message: "This page does not exist", code: "FRONT_RESOURCE_NOT_FOUND" });
}