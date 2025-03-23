import { gql, useMutation } from "@apollo/client";
import { useState } from "react";
import { ROOT_QUERY } from "./routes";

const POST_PHOTO_MUTATION = gql`
  mutation postPhoto($input: PostPhotoInput!) {
    postPhoto(input: $input) {
      id
      name
      url
    }
  }
`;

export const PostPhoto = () => {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState("PORTRAIT");
  const [file, setFile] = useState("");
  const updatePhotos = (cache, { data: { postPhoto } }) => {
    const data = cache.readQuery({ query: ROOT_QUERY });
    const allPhotos = [...data.allPhotos, postPhoto];
    cache.writeQuery({
      query: ROOT_QUERY,
      data: {
        ...data,
        allPhotos,
      },
    });
  };
  const [postPhotoMutation] = useMutation(POST_PHOTO_MUTATION, {
    update: updatePhotos,
  });

  const postPhoto = () => {
    postPhotoMutation({
      variables: {
        input: {
          name,
          description,
          category,
          file,
        },
      },
    });
  };

  return (
    <form
      onSubmit={(e) => e.preventDefault()}
      style={{
        display: "flex",
        flexDirection: "column",
        justifyContent: "flex-start",
        alignItems: "flex-start",
      }}
    >
      <h1>Post a Photo</h1>

      <input
        type="text"
        style={{ margin: "10px" }}
        placeholder="photo name..."
        value={name}
        onChange={({ target }) => setName(target.value)}
      />

      <textarea
        style={{ margin: "10px" }}
        placeholder="photo description..."
        value={description}
        onChange={({ target }) => setDescription(target.value)}
      />

      <select
        value={category}
        style={{ margin: "10px" }}
        onChange={({ target }) => setCategory(target.value)}
      >
        <option value="PORTRAIT">PORTRAIT</option>
        <option value="LANDSCAPE">LANDSCAPE</option>
        <option value="ACTION">ACTION</option>
        <option value="GRAPHIC">GRAPHIC</option>
      </select>

      <input
        type="file"
        style={{ margin: "10px" }}
        accept="image/jpeg"
        onChange={({ target }) =>
          setFile(target.files && target.files.length ? target.files[0] : "")
        }
      />

      <div style={{ margin: "10px" }}>
        <button type="button" onClick={postPhoto}>
          Post Photo
        </button>
      </div>
    </form>
  );
};
