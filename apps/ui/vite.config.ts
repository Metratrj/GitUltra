import { sentrySvelteKit } from "@sentry/sveltekit";
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, searchForWorkspaceRoot } from 'vite';

export default defineConfig({
    server: {
        fs: {
            allow: [
                // search up for workspace root
                searchForWorkspaceRoot(process.cwd())
            ]
        }
    },
	plugins: [sentrySvelteKit({
        sourceMapsUploadOptions: {
            org: "metratrj",
            project: "javascript-sveltekit"
        }
    }), sveltekit()]
});