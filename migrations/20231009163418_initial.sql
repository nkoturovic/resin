CREATE TABLE "users" (
"id" uuid PRIMARY KEY,
"username" character varying,
"email" character varying,
"password" character varying,
"first_name" character varying,
"last_name" character varying,
"date_of_birth" character varying,
"country" character varying,
"language" character varying,
"created_at" timestamptz,
"updated_at" timestamptz
)