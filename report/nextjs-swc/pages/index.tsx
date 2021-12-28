import type { NextPage } from "next";
import { useEffect, useState } from "react";
import { Sample } from "../components/Sample";
import { Status } from "../types";

const IndexPage: NextPage = () => {
  const [value, setValue] = useState<Status>("loading");
  useEffect(() => {
    setTimeout(() => {
      setValue("success");
    }, 1000);
  }, []);
  return <Sample value={value} />;
};

export default IndexPage;
