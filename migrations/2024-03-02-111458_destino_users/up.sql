-- Your SQL goes here
CREATE TABLE public.destino_users
(
    id uuid NOT NULL,
    fullname text NOT NULL,
    email text NOT NULL,
    phone_number bigint NOT NULL,
    joined date NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.destino_users
    OWNER to postgres;