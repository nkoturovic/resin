CREATE TABLE "users" (
"id" bigint NOT NULL PRIMARY KEY,
"username" character varying,
"email" character varying,
"password" character varying,
"first_name" character varying,
"last_name" character varying,
"date_of_birth" character varying,
"country" character varying,
"language" character varying,
"created_at" timestamptz NOT NULL,
"updated_at" timestamptz NOT NULL
)