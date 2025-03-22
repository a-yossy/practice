import { useMutation, gql, useQuery, useApolloClient } from "@apollo/client";
import { useNavigate } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { ROOT_QUERY } from "./routes";

const GITHUB_AUTH_MUTATION = gql`
  mutation githubAuth($code: String!) {
    githubAuth(code: $code) {
      token
    }
  }
`;

const CurrentUser = ({ name, avatar, logout }) => {
  return (
    <div>
      <img src={avatar} width={48} height={48} alt="" />
      <h1>{name}</h1>
      <button onClick={logout}>logout</button>
    </div>
  );
};

const Me = ({ logout, requestCode, signingIn }) => {
  const { loading, data } = useQuery(ROOT_QUERY);

  if (loading) return <p>loading...</p>;
  if (data.me === null)
    return (
      <button onClick={requestCode} disabled={signingIn}>
        Sign In with GitHub
      </button>
    );

  return <CurrentUser {...data.me} logout={logout} />;
};

export const AuthorizedUser = () => {
  const [signingIn, setSigningIn] = useState(false);
  const navigate = useNavigate();
  const client = useApolloClient();
  const authorizationComplete = (cache, { data }) => {
    localStorage.setItem("token", data.githubAuth.token);
    navigate({ to: "/" });
    setSigningIn(false);
  };
  const [githubAuth] = useMutation(GITHUB_AUTH_MUTATION, {
    update: authorizationComplete,
    refetchQueries: [{ query: ROOT_QUERY }],
  });

  const requestCode = () => {
    const clientID = process.env.FARM_CLIENT_ID;
    window.location = `https://github.com/login/oauth/authorize?client_id=${clientID}&scope=user&redirect_uri=http://localhost:9000`;
  };

  useEffect(() => {
    if (window.location.search.match(/code=/)) {
      setSigningIn(true);
      const code = window.location.search.replace("?code=", "");
      githubAuth({ variables: { code } });
    }
  }, [githubAuth]);

  return (
    <Me
      signingIn={signingIn}
      requestCode={requestCode}
      logout={() => {
        localStorage.removeItem("token");
        const data = client.readQuery({ query: ROOT_QUERY });
        data.me = null;
        client.writeQuery({ query: ROOT_QUERY, data: { ...data, me: null } });
      }}
    />
  );
};
