---
marp: true
paginate: true
# theme: # default | gaia | uncover
---

# Supabase について ![height:40](supabase-logo-icon.png)

#### 開発部 石川

---

# [Supabase](https://supabase.com/) とは

- いわゆる BaaS（Backend as a Service）と呼ばれるもの
- Firebase の RDB 版
- オープンソース、セルフホスティングも可能
- Web, App 対応

---

# 大まかな機能

- Database（PostgreSQL）
- Authentication
- Storage
- Realtime
- Logging
- Edge Funcitons （Beta ~ 2022/08/01）
- GraphQL

etc...

---

# サンプルコード（Web）を交えた各機能の解説

<!-- _footer: Flutter https://supabase.com/docs/guides/with-flutter -->

---

## :one: Supabase プロジェクトの初期化

- https://supabase.com から GitHub ログインして、プロジェクト名やパスワード、リージョン等を設定。
- 設定後、https://app.supabase.com/project/<projectId> から、API エンドポイントと anon key をコピー

<!-- LT用 https://app.supabase.com/project/nlemkqykrspauaasknga -->

---

### テーブル作成

- SQL Editor か、Table Editor 上でテーブルを定義
- auth.users テーブルがデフォルトで定義されており、これは変更不可
  - 直接読み取ることもできないので、少し工夫が必要になる
- public スキーマ上に定義する

![bg right:45%](table-editor-1.png)

<!-- https://app.supabase.com/project/nlemkqykrspauaasknga/editor -->

---

## :two: SDK のインストール、クライアントの初期化

```bash
npm i @supabase/supabase-js
```

```TypeScript
import { createClient } from "@supabase/supabase-js";

const SUPABASE_URL = process.env.NEXT_PUBLIC_SUPABASE_URL;
const SUPABASE_ANON_KEY = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

export const supabase = createClient(SUPABASE_URL, SUPABASE_ANON_KEY);
```

---

## :three: 認証

```TypeScript
import { supabase } from "どっか"; // 2で初期化したインスタンスをimport

// ユーザー情報を取得
const user = supabase.auth.user();

// ログイン
const { user, session, error } = await supabase.auth.signIn({
  email: 'example@email.com',
  password: 'example-password',
});
```

- SMS 認証、email による招待なども可能
- プロバイダーも豊富
  - Apple,Discord,Facebook,GitHub,Google,Notion,Twitter...

---

## :four: データ取得

すべての tasks テーブルのデータを取得

```TypeScript
type Task = { id: string, ... }
const { data, error } = await supabase.from<Task>("tasks").select("*")
// .eq({ user_id: "hoge" })
```

コメント部分のようにフィルターも可能だが、後述する RLS とどちらを使用するかは要検討

<!-- _footer: SDK側でキャッシュはしてないので注意 -->

---

### Realtime

- DB 側でデータに変更があった場合に、WebSocket を用いてクライアント側へデータを送信することができる
- ダッシュボードから、適用したいテーブルの設定をオンにすることで使用可能

```TypeScript
const tasks = supabase
  .from('tasks')
  .on('*', payload => { // INSERT | DELETE...
    console.log('Change received!', payload)
  })
  .subscribe()
```

---

## :five: 作成、更新、削除

```TypeScript
const { data, error } = await supabase
  .from('tasks')
  .insert([{ title: 'foo' }], { upsert: true })

const { data, error } = await supabase
  .from('tasks')
  .update({ title: 'hoge' })
  .eq('id', 'taskId')

const { data, error } = await supabase
  .from('tasks')
  .delete()
  .eq('id', 'taskId')
```

---

## :six: RLS（Row Level Security）

- PostgreSQL の機能で、ポリシーを指定することで行単位でのセキュリティの設定が可能
- ポリシー
  - 「各ポリシーはテーブルに関連付けられ、テーブルにアクセスするたびにポリシーが実行される。すべてのクエリに WHERE 句を追加するようなもの」とのこと
- Supabase 側で、ポリシーで使用できる便利関数がいくつか用意されている。
  - `auth.uid()` -> リクエストを行ったユーザーの ID を返す、など

---

### 具体例

```SQL
CREATE POLICY "policy_name"
ON public.tasks
FOR SELECT USING (
  auth.uid() = user_id
);
```

↑ tasks テーブルを Read したときに、リクエストを行ったユーザー ID と tasks.user_id が一致するレコードしか返さない

**細かい認証ルールが必要な場合には、基本的にこの RLS のポリシーで設定する**

<!-- _footer: クライアント側で自由に DB へアクセスされるのを防ぐためにも、基本的に RLS の設定は必須-->

---

- Database Functions
  - SQL で定義
  - supabase.rpc("関数名")
- Edge Functions
  - Deno ランタイム上で JS / TS を実行可能

---

## おまけ（GraphQL）

- Supabase で GraphQL も利用可能に :tada:
- ダッシュボード で少しポチポチして、SQL を実行するだけですぐ使える
- 自動で各テーブルの Query, Mutation を生成

```SQL
comment on schema public is e'@graphql({"inflect_names": true})';
```

```SQL
select graphql.rebuild_schema();
```

<!-- https://nlemkqykrspauaasknga.supabase.co/graphql/v1 -->
<!-- header apiKey: anon key -->

---

## その他

- トランザクションや、特定のデータの変更を感知して処理を実行したい場合などは、`Database Functions`や`Triggers`を使えばできそう
<!-- 参考:
- https://zenn.dev/hrtk/articles/supabase-nextjs-database-function-table
- https://zenn.dev/matken/articles/use-user-info-with-supabase
  -->

- 全文検索も可能
- 匿名ログインができない
  - → Email 認証の設定で、メールを確認してなくても登録可にすることでそれっぽくはできる。
- `supabase-cli`
  - マイグレーション
  - Docker を使ったローカル開発
  <!-- - `Edge Functios` は Deno ランタイム上で JS,TS を実行できる（ユースケースがまだあんまわかってない） -->

---

# 所感

---

# :smile:

- 爆速でバックエンド作れて幸せ
  - Next.js on Vercel + Supabase、やべぇ
- API も直感的でわかりやすく、GraphQL も使える
- 個人、小規模のプロダクトなら全然 Supabase で良さそう
- Twitter で「Supabase」っていれてつぶやくと、中の人がたまに反応してくれる（質問答えてくれた）

---

# :weary:

- ダッシュボードからボタンぽちぽちするだけで完結するのは厳しそう。SQL の知識は必要だと感じた
  - Enum、RLS、Delete Cascade など
- すべての機能が Production Ready なわけではないので、注意は必要
- Web Push がない

---

# ご清聴ありがとうございました
