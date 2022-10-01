import { ApolloClient, ApolloProvider, InMemoryCache } from "@apollo/client";
import type { AppProps } from "next/app";
import { FC } from "react";

const App: FC<AppProps> = ({ Component, pageProps }) => {
  return (
    <ApolloProvider
      client={
        new ApolloClient({
          uri: process.env.NEXT_PUBLIC_SUPABASE_URL + "/graphql/v1",
          headers: { apiKey: process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY },
          cache: new InMemoryCache(),
        })
      }
    >
      <Component {...pageProps} />
    </ApolloProvider>
  );
};

export default App;
