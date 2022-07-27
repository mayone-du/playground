import { FC, useState } from "react";
import { supabase } from "../lib/supabase/client";

export const CreateTaskForm: FC = () => {
  const [title, setTitle] = useState("");
  return (
    <form
      onSubmit={async (e) => {
        e.preventDefault();
        const { data, error } = await supabase
          .from("tasks")
          .insert([{ title }]);
        if (error) return alert(error.message);
      }}
    >
      <input
        type="text"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
      />
      <button type="submit">作成</button>
    </form>
  );
};
