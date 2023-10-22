import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { verifyEmail } from '$lib/api';

export const load: PageLoad = async ({ params }) => {
  if (!params.code) {
    return error(404, "Not found");
  }
  if (await verifyEmail(params.code)) {
    alert("Email verified!");
    return {
      status: 302,
      redirect: "/"
    };
  } else {
    return error(404, "Not found");
  }
};
