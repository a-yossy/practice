import "./index.css";
import { ApolloClient, InMemoryCache, ApolloProvider } from "@apollo/client";
import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { persistCache } from "apollo3-cache-persist";

import { routeTree } from "./routeTree.gen";

const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

const rootElement = document.getElementById("root");
if (rootElement !== null && !rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  const cache = new InMemoryCache();
  persistCache({
    cache,
    storage: localStorage,
  });
  const client = new ApolloClient({
    uri: "http://localhost:8000",
    cache,
    headers: {
      Authorization: localStorage.getItem("token"),
    },
  });
  if (localStorage['apollo-cache-persist']) {
    const cacheData = JSON.parse(localStorage['apollo-cache-persist'])
    cache.restore(cacheData)
  }

  root.render(
    <StrictMode>
      <ApolloProvider client={client}>
        <RouterProvider router={router} />
      </ApolloProvider>
    </StrictMode>
  );
}
