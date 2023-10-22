import { writable, type Writable } from 'svelte/store';
import type { User } from '$lib/models';

export enum SiteMode {
  Default,
  Login,
  SignUp,
  ResetPassword,
}

export const currentUser: Writable<User | null> = writable(null);
export const siteMode = writable(SiteMode.Default);
export const alertVisible = writable(false);



