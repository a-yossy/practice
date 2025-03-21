import { request } from "graphql-request";
import React from "react";
import { createRoot } from "react-dom/client";

const url = "http://localhost:8000";
const query = `
  query listUsers {
    allUsers {
      avatar
      name
    }
  }
`

const mutation = `
  mutation populate($count: Int!) {
    addFakeUsers(count:$count) {
      githubLogin
    }
  }
`

const App = ({ users=[] }) =>
  <div>
    {users.map(user =>
      <div key={user.githubLogin}>
        <img src={user.avatar} alt="" />
        {user.name}
      </div>
    )}
    <button type="button" onClick={addUser}>Add User</button>
  </div>

const render = ({ allUsers=[] }) => {
  const container = document.getElementById('root');
  const root = createRoot(container);
  root.render(
    <App users={allUsers} />
  );
}

const addUser = () =>
  request(url, mutation, {count:1})
    .then(requestAndRender)
    .catch(console.error)

const requestAndRender = () =>
  request(url, query)
    .then(render)
    .catch(console.error)

requestAndRender()
