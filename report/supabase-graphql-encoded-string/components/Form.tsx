import { FC, useState } from "react";
import { useAddTaskMutation } from "../graphql/generated/schema";
import { supabase } from "../utils/supabaseClient";

type Props = {
  type: "sdk" | "graphql";
  refetch: VoidFunction;
};

export const Form: FC<Props> = ({ type, refetch }) => {
  const [val, setVal] = useState("");
  const [err, setErr] = useState<null | string>(null);
  const [addTask] = useAddTaskMutation();

  const handleSubmit = async () => {
    if (!val) return;
    try {
      if (type === "sdk") {
        const { error } = await supabase.from("tasks").insert([{ title: val }]);
        error && setErr(JSON.stringify(error));
      } else {
        await addTask({ variables: { object: { title: val } } });
      }
      refetch();
      setVal("");
      setErr(null);
    } catch (e) {
      if (!(e instanceof Error)) return;
      setErr(JSON.stringify(e));
    }
  };

  return (
    <div>
      <div style={{ fontWeight: "bold" }}>{type}</div>
      <input onChange={(e) => setVal(e.target.value)} value={val} />
      <button onClick={handleSubmit}>Submit</button>
      {err && <div style={{ color: "red" }}>{err}</div>}
    </div>
  );
};
