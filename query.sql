create table users(
	id serial primary key,
	name varchar(50) not null,
	username varchar(50) unique not null,
	password text not null,
  age int not null,
	created_at timestamp default now()
);
