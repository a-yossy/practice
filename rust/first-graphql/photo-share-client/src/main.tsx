import "./main.css";
import {
  ApolloClient,
  InMemoryCache,
  ApolloProvider,
  gql,
} from "@apollo/client";
import { Users } from "./Users";
export function Main() {
  const client = new ApolloClient({
    uri: "http://localhost:8000",
    cache: new InMemoryCache(),
  });

  console.log({ cache: client.extract() });
  const query = gql`
    {
      totalUsers
      totalPhotos
    }
  `;
  client
    .query({ query })
    .then(() => console.log({ cache: client.extract() }))
    .then(console.error);

  return (
    <ApolloProvider client={client}>
      <Users />
    </ApolloProvider>
  );
}
