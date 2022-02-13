import dayjs from "dayjs";
import { useNewsListQuery } from "../graphql/schemas/generated/schema";

const IndexPage = () => {
  const { data, loading, error } = useNewsListQuery({
    variables: {
      input: { sharedAt: dayjs() },
    },
  });
  console.log("hoge", data, loading, error);
  return (
    <div>
      {/* {data?.newsList.map((news) => (
        <div key={news.nodeId}>{news.title}</div>
      ))} */}
    </div>
  );
};

export default IndexPage;
