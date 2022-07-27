import type { NextPage } from "next";
import { AuthForm } from "../components/AuthForm";
import { CreateTaskForm } from "../components/CreateTaskForm";
import { TaskList } from "../components/TaskList";
import { supabase } from "../lib/supabase/client";

const Home: NextPage = () => {
  const user = supabase.auth.user();

  return (
    <div>
      <h1>ToDoアプリで学ぶSupabase</h1>

      {user ? (
        <div>
          <CreateTaskForm />
          <TaskList />
        </div>
      ) : (
        <AuthForm />
      )}
    </div>
  );
};

export default Home;
