drop table if exists comments;
drop table if exists pr;
drop table if exists repository;

create table if not exists repository (
    pid serial not null constraint repository_pkey primary key,
    owner varchar(255) not null,
    repository varchar(255) not null,
    created_at timestamp default now() not null,
    updated_at timestamp default now() not null
);

create table if not exists pr (
    pid serial not null constraint pr_pkey primary key,
    repository_id integer not null references repository(pid),
    number integer not null unique,
    title varchar(255) not null unique,
    body text not null unique,
    endpoint varchar(255) not null unique,
    created_at timestamp default now() not null,
    updated_at timestamp default now() not null
);

create table if not exists comments (
    pid serial not null constraint comments_pkey primary key,
    pr_id integer not null references pr(pid),
    number integer not null unique,
    endpoint varchar(255) not null unique,
    body text not null unique,
    diff_hunk text not null unique,
    path varchar(255) not null unique,
    html_url varchar(255) not null unique,
    created_at timestamp default now() not null,
    updated_at timestamp default now() not null
);