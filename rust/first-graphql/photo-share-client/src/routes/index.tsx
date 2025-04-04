import "../main.css";
import { createFileRoute } from "@tanstack/react-router";
import { gql, useMutation, useQuery } from "@apollo/client";
import { AuthorizedUser } from "../AuthorizedUser";
import { PostPhoto } from "../UploadFile";

export const ROOT_QUERY = gql`
  query allUsers {
    totalUsers
    allUsers {
      ...userInfo
    }
    me {
      ...userInfo
    }
    allPhotos {
      id
      name
      url
    }
  }

  fragment userInfo on User {
    githubLogin
    name
    avatar
  }
`;

const ADD_FAKE_USERS_MUTATION = gql`
  mutation addFakeUsers($count: Int!) {
    addFakeUsers(count: $count) {
      githubLogin
      name
      avatar
    }
  }
`;

const Photos = ({ allPhotos }) => {
  return (
    <>
      {allPhotos.map((photo) => {
        return (
          <img key={photo.id} src="" alt={photo.name} width={350} />
        );
      })}
    </>
  );
};

const UserList = ({ count, users, refetchUsers }) => {
  const [addFakeUsers] = useMutation(ADD_FAKE_USERS_MUTATION, {
    variables: { count: 1 },
  });

  return (
    <div>
      <AuthorizedUser />
      <p>{count} Users</p>
      <button onClick={() => refetchUsers()}>Refetch Users</button>
      <button onClick={addFakeUsers}>Add Fake Users</button>
      <ul>
        {users.map((user) => (
          <UserListItem
            key={user.githubLogin}
            name={user.name}
            avatar={user.avatar}
          />
        ))}
      </ul>
    </div>
  );
};

const UserListItem = ({ name, avatar }) => (
  <li>
    <img src={avatar} width={48} height={48} alt="" />
    {name}
  </li>
);

const Index = () => {
  const { loading, error, data, refetch } = useQuery(ROOT_QUERY, {
    fetchPolicy: "cache-and-network",
  });

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  return (
    <>
      <PostPhoto />
      <UserList
        count={data.totalUsers}
        users={data.allUsers}
        refetchUsers={refetch}
      />
      <Photos allPhotos={data.allPhotos} />
    </>
  );
};

export const Route = createFileRoute("/")({
  component: Index,
});
