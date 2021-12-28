import type { NextPage } from "next";
import { useEffect, useState } from "react";

type Status = "success" | "error" | "loading";

const IndexPage: NextPage = () => {
  const [state, setState] = useState<Status>("loading");

  useEffect(() => {
    setTimeout(() => {
      setState("success");
    }, 3000);
  }, []);

  if (state === "loading") {
    return <div>Loading...</div>;
  } else if (state === "error") {
    return <div>Error!!</div>;
  } else {
    return <div>Success!!!</div>;
  }

  return <div>unreachable!!!</div>;
};

export default IndexPage;
