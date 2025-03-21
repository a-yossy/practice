import { defineConfig } from "@farmfe/core";
import { TanStackRouterVite } from "@tanstack/router-plugin/vite";

export default defineConfig({
  plugins: ["@farmfe/plugin-react"],
  vitePlugins: [
    TanStackRouterVite({ target: "react", autoCodeSplitting: false }),
  ],
});
