import { defineConfig } from "astro/config";
import tailwind from "@astrojs/tailwind";
import react from "@astrojs/react";
import icon from "astro-icon";

import vercel from "@astrojs/vercel/serverless";

// https://astro.build/config
export default defineConfig({
	output: "server",
	adapter: vercel(),
	integrations: [
		icon({
			mdi: ["*"],
			uis: ["*"],
		}),
		tailwind(),
		react(),
	],
});
