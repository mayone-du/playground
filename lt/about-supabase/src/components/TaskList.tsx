import { FC, useEffect, useState } from "react";
import { supabase } from "../lib/supabase/client";
import { Task } from "../types/task";

export const TaskList: FC = () => {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    (async () => {
      const { data } = await supabase.from<Task>("tasks").select("*");
      setTasks(data ?? []);
    })();
  }, []);

  return (
    <div>
      <ul>
        {tasks.map(({ id, title }) => {
          return <li key={id}>{title}</li>;
        })}
      </ul>
    </div>
  );
};
