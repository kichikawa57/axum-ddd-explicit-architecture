-- Add migration script here
create table if not exists users (
    id varchar(26) not null,
    name varchar(255) not null,
    created_at timestamp with time zone not null default current_timestamp,
    updated_at timestamp with time zone not null default current_timestamp,
    constraint pk_users_id primary key (id)
);