import { NextPage } from "next";
import { useState } from "react";

const IndexPage: NextPage = () => {
  const [sdkInput, setSdkInput] = useState("");
  const [graphqlInput, setGraphqlInput] = useState("");

  const handleSDKSubmit = async () => {};
  const handleGraphQLSubmit = async () => {};

  return (
    <div>
      <form>
        <input
          style={{
            display: "block",
            border: "1px solid lightgray",
            padding: "4px",
          }}
        />
        <button style={{ display: "block" }}>submit</button>
      </form>
    </div>
  );
};

export default IndexPage;
