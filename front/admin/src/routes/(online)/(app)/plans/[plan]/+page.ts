import { error } from '@sveltejs/kit';

export const load = (async ({ parent, params }) => {
  const { plans } = await parent();
  const plan = plans.items.find(item => item._id === params.plan);
  if(plan == null) {
    throw error(404, { status: 404, code: "PLAN_NOT_FOUND", message: `Plan with id ${params.plan} not found` });
  }

  return { plan }
}) satisfies import("./$types").PageLoad;