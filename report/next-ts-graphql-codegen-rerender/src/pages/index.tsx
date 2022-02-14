// import dayjs from "dayjs";
import {
  useNewsListQuery,
  useTestQuery,
} from "../graphql/schemas/generated/schema";

const IndexPage = () => {
  const { data, loading, error } = useTestQuery({
    variables: { customArg: new Date() },
  });
  // const { data, loading, error } = useNewsListQuery({
  //   variables: {
  //     input: { sharedAt: new Date() },
  //   },
  // });
  console.log("hoge", data, loading, error);
  return (
    <div>
      {data?.test}
      {/* {data?.newsList.map((news) => (
        <div key={news.nodeId}>{news.title}</div>
      ))} */}
    </div>
  );
};

export default IndexPage;
