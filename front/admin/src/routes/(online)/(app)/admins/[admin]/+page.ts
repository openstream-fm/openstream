import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params }) => {
  
  const { admins } = await parent();

  const page_admin = admins.items.find(item => item._id === params.admin);

  if(page_admin == null) throw error(404, { status: 404, message: "Admin not found", code: "FRONT_RESOURCE_NOT_FOUND" })

  return { page_admin }

}) satisfies import("./$types").PageLoad;