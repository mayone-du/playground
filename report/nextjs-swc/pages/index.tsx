import type { NextPage } from "next";
import { useEffect, useState } from "react";

type Status = "loading" | "error" | "success";

const IndexPage: NextPage = () => {
  const [value, setValue] = useState<Status>("loading");
  useEffect(() => {
    setTimeout(() => {
      setValue("success");
    }, 1000);
  }, []);

  if (value === "loading") {
    return <div>loading...</div>;
  } else if (value === "error") {
    return <div>error</div>;
  } else {
    return <div>success🥰 unreachable code is not called🙆‍♂️</div>;
  }

  return <div>unreachable code!!! this is a bug🐛</div>;
};

export default IndexPage;
