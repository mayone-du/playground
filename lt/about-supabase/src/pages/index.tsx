import type { NextPage } from "next";
import { useState } from "react";

const Home: NextPage = () => {
  const [title, setTitle] = useState("");
  return (
    <main>
      <h1>ToDoアプリで学ぶSupabase</h1>
      <form
        onSubmit={(e) => {
          e.preventDefault();
        }}
      >
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
        />
        <button type="submit">作成</button>
      </form>
    </main>
  );
};

export default Home;
