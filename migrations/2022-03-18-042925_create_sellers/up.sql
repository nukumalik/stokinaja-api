create table sellers (
  id varchar(100) primary key default(uuid()),
  name varchar(50) not null,
  address text,
  email varchar(50) not null unique,
  password varchar(100) not null,
  phone varchar(20),
  created_at timestamp not null default current_timestamp,
  upadted_at timestamp not null default current_timestamp on update current_timestamp
);