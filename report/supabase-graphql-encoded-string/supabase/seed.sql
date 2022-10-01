-- table:tasks
create table public.tasks (
  id uuid primary key not null default uuid_generate_v4(),
  title text not null default ''
);

-- build graphql schema
comment on schema public is e'@graphql({"inflect_names": true})';

select
  graphql.rebuild_schema();