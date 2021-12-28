import { Box, TextField } from "@mui/material";
import { VFC } from "react";
import { Status } from "../types";

type Props = {
  value: Status;
};

export const Sample: VFC<Props> = (props) => {
  if (props.value === "loading") {
    return <Box>status is Loading...</Box>;
  } else if (props.value === "error") {
    return <Box>status is Error!</Box>;
  } else {
    return <Box>else block {props.value}</Box>;
  }
  return <Box>unreachable block!!!!</Box>;
};
