create table companies (
  id varchar(100) primary key default(uuid()),
  name varchar(100) not null,
  address text,
  description text,
  email varchar(50) not null,
  phone varchar(20) not null,
  created_at timestamp not null default current_timestamp,
  upadted_at timestamp not null default current_timestamp on update current_timestamp
);