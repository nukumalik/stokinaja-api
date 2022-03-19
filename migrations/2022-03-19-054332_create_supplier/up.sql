create table suppliers (
  id varchar(100) primary key default(uuid()),
  name varchar(100) not null,
  email varchar(50) not null unique,
  password varchar(100) not null,
  phone varchar(20),
  company_id varchar(100) not null,
  created_at timestamp not null default current_timestamp,
  upadted_at timestamp not null default current_timestamp on update current_timestamp,
  foreign key(company_id) REFERENCES companies(id)
);