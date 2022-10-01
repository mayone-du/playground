import { NextPage } from "next";
import { Form } from "../components/Form";
import { useTasksQuery } from "../graphql/generated/schema";

const IndexPage: NextPage = () => {
  const { data, refetch } = useTasksQuery();
  return (
    <div>
      <ul>
        {data?.tasksCollection?.edges.map(({ node }) => {
          return (
            <li key={node.id}>
              {node.id}:{node.title}
            </li>
          );
        })}
      </ul>
      <div style={{ display: "flex", gap: "30px" }}>
        <Form type="sdk" refetch={refetch} />
        <Form type="graphql" refetch={refetch} />
      </div>
    </div>
  );
};

export default IndexPage;
