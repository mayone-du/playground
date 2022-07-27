import { FC, useState } from "react";
import { supabase } from "../lib/supabase/client";

export const AuthForm: FC = () => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  return (
    <form
      onSubmit={async (e) => {
        e.preventDefault();
        const { user, error } = await supabase.auth.signUp({
          email,
          password,
        });
        if (error) return alert(error.message);
        supabase.auth.signIn({ email, password });
      }}
    >
      <label>
        email
        <input
          type="text"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          style={{ display: "block" }}
        />
      </label>

      <label>
        password
        <input
          type="text"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          style={{ display: "block" }}
        />
      </label>
      <button type="submit">登録</button>
    </form>
  );
};
