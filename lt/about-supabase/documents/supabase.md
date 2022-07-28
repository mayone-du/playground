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
- オープンソース、セルフホスティング可
- Web, App 対応

---

# 大まかな機能

- Database（PostgreSQL）
- Authentication
- Storage
- Realtime
- Edge Funcitons （Beta ~ 2022/08/01）
- GraphQL

etc...

---

# サンプルコード（Web）や各機能について

<!-- _footer: Flutter https://supabase.com/docs/guides/with-flutter -->

---

## :one: Supabase プロジェクトの初期化

- https://supabase.com から GitHub ログイン
- プロジェクト名やパスワード、リージョン等を設定
  - 東京リージョン :o:

<!-- LT用 https://app.supabase.com/project/nlemkqykrspauaasknga -->

---

### テーブル作成

- SQL Editor か、Table Editor でテーブルを定義
  - UI の実装がまだ追いついていない
  - 細かい設定をしたい場合は SQL を書く必要がある
- Supabase で認証を行うと、auth.users テーブルにデータがたまってく

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

- anon
  - クライアントへ渡して OK
- service_role
  - サーバー側でしか使用してはいけない。RLS の設定を無視できる管理者用のキー

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

- SMS 認証
- ソーシャルログイン
  - Apple, Discord, Facebook, GitHub, Google, Notion, Twitter...

---

## :four: データ取得

すべての tasks テーブルのデータを取得

```TypeScript
type Task = { id: string, title: string, ... };
const { data, error } = await supabase.from<Task>("tasks").select("*");
// .eq({ user_id: "hoge" })
// data: Task[]
```

コメント部分のようにフィルターも可能だが、後述する RLS とどちらを使用するかは要検討

<!-- _footer: SDK側でキャッシュはしてないので注意 -->

---

### Realtime

- DB に変更があった時に、WebSocket を用いてクライアント側へデータを送信することができる
- ダッシュボードから、有効化したいテーブルを選ぶことで使用可能

```TypeScript
const tasksListener = supabase
  .from('tasks')
  .on('*', payload => { // INSERT | DELETE...
    console.log('Change received!', payload)
  })
  .subscribe();

// tasksListener.unsubscribe();
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
- Supabase 側で、ポリシーで使用できる[便利関数](https://supabase.com/docs/guides/auth/row-level-security#helper-functions)がいくつか用意されている
  - `auth.uid()` -> リクエストを行ったユーザーの ID を返す、など

---

### 具体例

```SQL
CREATE POLICY "policy_name"
ON public.tasks
FOR SELECT USING ( -- 既存レコードには USING、新規レコードには WITH CHECK
  auth.uid() = user_id -- 条件式
);
```

↑ tasks テーブルを取得したときに、リクエストを行ったユーザー ID と tasks.user_id が一致する場合のみ OK

**細かいアクセス制限が必要な場合には、基本的にこの RLS のポリシーで設定する**

<!-- _footer: クライアント側で自由に DB へアクセスされるのを防ぐためにも、基本的に RLS の有効化は必須-->

<!-- ---
- Database Functions
  - SQL で定義
  - supabase.rpc("関数名")
  - triggerを作成して、〇〇テーブルへのinsertをトリガーに関数実行、みたいなことができる
- Edge Functions
  - Deno ランタイム上で JS / TS を実行可能 -->

---

## :seven: GraphQL

- Supabase で GraphQL も利用可能に :tada:
- 有効化するにはダッシュボードでボタンおすだけ
- 自動で各テーブルの Query, Mutation を生成

```SQL
comment on schema public is e'@graphql({"inflect_names": true})';
-- snake_case → PascalCase
```

```SQL
select graphql.rebuild_schema(); -- GraphQLスキーマを再生成
```

<!-- https://nlemkqykrspauaasknga.supabase.co/graphql/v1 -->
<!-- header apiKey: anon key -->

---

## その他 :one:

- `Database Functions`や`Triggers`
  - トランザクション
  - 特定のデータの変更を感知して処理を実行
  - `Database Funcitons`はクライアント側から呼び出すことも可能（`supabase.rpc("関数名")`）
- `Edge Functios`
  - Deno ランタイム上で JS,TS を実行できる

DB をいじりたい場合 → `Database Functions`
低レイテンシーを必要とする場合 → `Edge Functions`

  <!-- 参考: - https://zenn.dev/hrtk/articles/supabase-nextjs-database-function-table - https://zenn.dev/matken/articles/use-user-info-with-supabase
  -->

---

## その他 :two:

- `supabase-cli`
  - Docker を使ったローカル開発
  - マイグレーション
  - `Edge Functions`の作成、デプロイ

---

# 所感

---

# :smile:

- 手早くバックエンド作れて幸せ
- API も直感的でわかりやすく、GraphQL も使える
- 主要な機能は大体揃ってるし、PostgreSQL の機能を使えるため、結構色々できる
- Twitter で「Supabase」についてつぶやくと、中の人がたまに反応してくれる（質問答えてくれた）

---

# :weary:

- SQL の学習コスト
  - テーブル定義、RLS、Database Functions など
- すべての機能が Production Ready なわけではないので注意
- Web Push がない

---

**全体的に好印象**

- 個人、小規模のプロダクト
- SQL の学習コスト < バックエンド 1 から作る or 別サービス

---

# ご清聴ありがとうございました :wave:
