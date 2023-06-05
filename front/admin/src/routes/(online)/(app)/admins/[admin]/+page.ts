import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params }) => {
  
  const { admins } = await parent();

  const admin = admins.items.find(item => item._id === params.admin);

  if(admin == null) throw error(404, { status: 404, message: "Admin not found", code: "FRONT_RESOURCE_NOT_FOUND" })

  return { admin }

}) satisfies import("./$types").PageLoad;