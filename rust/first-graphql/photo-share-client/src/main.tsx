import "./index.css";
import {
  ApolloClient,
  InMemoryCache,
  ApolloProvider,
  split,
  gql,
  useApolloClient,
} from "@apollo/client";
import { StrictMode, useEffect } from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { persistCache } from "apollo3-cache-persist";
import { setContext } from "@apollo/client/link/context";
import { createClient } from "graphql-ws";
import { GraphQLWsLink } from "@apollo/client/link/subscriptions";
import createUploadLink from "apollo-upload-client/createUploadLink.mjs";

import { routeTree } from "./routeTree.gen";
import { getMainDefinition } from "@apollo/client/utilities";
import { ROOT_QUERY } from "./routes";

const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

const LISTEN_FOR_USERS = gql`
  subscription {
    newUser {
      githubLogin
      name
      avatar
    }
  }
`;

const App = () => {
  const client = useApolloClient();
  useEffect(() => {
    const subscription = client
      .subscribe({ query: LISTEN_FOR_USERS })
      .subscribe(({ data: { newUser } }) => {
        const data = client.readQuery({ query: ROOT_QUERY });
        client.writeQuery({
          query: ROOT_QUERY,
          data: {
            ...data,
            totalUsers: data.totalUsers + 1,
            allUsers: [...data.allUsers, newUser],
          },
        });
      });

    return () => {
      subscription.unsubscribe();
    };
  }, [client]);

  return <></>;
};

const rootElement = document.getElementById("root");
if (rootElement !== null && !rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  const cache = new InMemoryCache();
  const httpLink = createUploadLink({
    uri: "http://localhost:8000",
  });
  const authLink = setContext((_, { headers }) => {
    const token = localStorage.getItem("token");
    return {
      headers: {
        ...headers,
        Authorization: token ? token : "",
      },
    };
  });
  const httpAuthLink = authLink.concat(httpLink);
  const wsLink = new GraphQLWsLink(
    createClient({
      url: "ws://localhost:8000/ws",
    })
  );
  const splitLink = split(
    ({ query }) => {
      const definition = getMainDefinition(query);
      return (
        definition.kind === "OperationDefinition" &&
        definition.operation === "subscription"
      );
    },
    wsLink,
    httpAuthLink
  );

  persistCache({
    cache,
    storage: localStorage,
  });
  const client = new ApolloClient({
    link: authLink.concat(splitLink),
    cache,
  });
  if (localStorage["apollo-cache-persist"]) {
    const cacheData = JSON.parse(localStorage["apollo-cache-persist"]);
    cache.restore(cacheData);
  }

  root.render(
    <StrictMode>
      <ApolloProvider client={client}>
        <App />
        <RouterProvider router={router} />
      </ApolloProvider>
    </StrictMode>
  );
}
