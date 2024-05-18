// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
import { User } from "./lib/types";

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: User;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
