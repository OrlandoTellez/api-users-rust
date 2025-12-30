create type valid_genders as enum ('male', 'female', 'other', 'prefer_not_to_say');

-- tabla de usuarios
create table users(
	id serial primary key,
	name varchar(50) not null,
	username varchar(50) unique not null,
	password text not null,
  age int not null,
  gender valid_genders not null,
	created_at timestamp default now()
);
