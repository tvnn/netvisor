--
-- PostgreSQL database dump
--

\restrict vxk1ZN9GUoaaFGV9qtb4fAelSj1pfjUNVq2QKmbK1rg2x7oU7Bguko7UlltLpHG

-- Dumped from database version 17.6
-- Dumped by pg_dump version 17.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO postgres;

--
-- Name: daemons; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.daemons (
    id uuid NOT NULL,
    network_id uuid NOT NULL,
    host_id uuid NOT NULL,
    ip text NOT NULL,
    port integer NOT NULL,
    registered_at timestamp with time zone NOT NULL,
    last_seen timestamp with time zone NOT NULL
);


ALTER TABLE public.daemons OWNER TO postgres;

--
-- Name: groups; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.groups (
    id uuid NOT NULL,
    network_id uuid NOT NULL,
    name text NOT NULL,
    description text,
    group_type text NOT NULL,
    service_bindings jsonb NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    source jsonb NOT NULL
);


ALTER TABLE public.groups OWNER TO postgres;

--
-- Name: hosts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.hosts (
    id uuid NOT NULL,
    network_id uuid NOT NULL,
    name text NOT NULL,
    hostname text,
    description text,
    target jsonb NOT NULL,
    interfaces jsonb,
    services jsonb,
    ports jsonb,
    source jsonb NOT NULL,
    virtualization jsonb,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


ALTER TABLE public.hosts OWNER TO postgres;

--
-- Name: networks; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.networks (
    id uuid NOT NULL,
    name text NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    is_default boolean NOT NULL,
    user_id uuid NOT NULL
);


ALTER TABLE public.networks OWNER TO postgres;

--
-- Name: services; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.services (
    id uuid NOT NULL,
    network_id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    name text NOT NULL,
    host_id uuid NOT NULL,
    bindings jsonb,
    service_definition text NOT NULL,
    virtualization jsonb,
    source jsonb NOT NULL
);


ALTER TABLE public.services OWNER TO postgres;

--
-- Name: subnets; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.subnets (
    id uuid NOT NULL,
    network_id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    cidr text NOT NULL,
    name text NOT NULL,
    description text,
    subnet_type text NOT NULL,
    source jsonb NOT NULL
);


ALTER TABLE public.subnets OWNER TO postgres;

--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id uuid NOT NULL,
    name text NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public._sqlx_migrations (version, description, installed_on, success, checksum, execution_time) FROM stdin;
20251006215000	users	2025-10-23 23:00:20.228745+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	4711500
20251006215100	networks	2025-10-23 23:00:20.234308+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	3510333
20251006215151	create hosts	2025-10-23 23:00:20.238149+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	2718458
20251006215155	create subnets	2025-10-23 23:00:20.241171+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	2725042
20251006215201	create groups	2025-10-23 23:00:20.244208+00	t	\\x96cdc35b7ad03869a836d4a4fe8c3060d075c32edce248827903ceab5c4e41b0727300d6c5755e54973f3ada9e50293a	2668709
20251006215204	create daemons	2025-10-23 23:00:20.247209+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	2774458
20251006215212	create services	2025-10-23 23:00:20.250299+00	t	\\xd5b07f82fc7c9da2782a364d46078d7d16b5c08df70cfbf02edcfe9b1b24ab6024ad159292aeea455f15cfd1f4740c1d	2559958
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
\.


--
-- Data for Name: groups; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.groups (id, network_id, name, description, group_type, service_bindings, created_at, updated_at, source) FROM stdin;
\.


--
-- Data for Name: hosts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.hosts (id, network_id, name, hostname, description, target, interfaces, services, ports, source, virtualization, created_at, updated_at) FROM stdin;
97d10182-da62-457f-ac70-137275c5718b	e861b2be-2cc4-4cb5-86ae-febd936afd2a	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "1ff736d4-9ef1-43c1-aa3e-914523d499a7"}	[{"id": "963b4331-37c4-422e-9214-881369f9ae8d", "name": "Internet", "subnet_id": "b4abcbcd-17ee-458a-a623-b307e5708a4e", "ip_address": "1.1.1.1", "mac_address": null}]	["2a8efd06-5bf7-4cac-b6d5-acc498a9a721"]	[{"id": "6919de89-401d-4ce0-97de-ceee5106e6ec", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-23 23:00:49.165106+00	2025-10-23 23:00:49.178894+00
e309d76c-1a29-4a3d-884d-d88010220a5b	e861b2be-2cc4-4cb5-86ae-febd936afd2a	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "76d97576-986f-4e9a-9642-94ebc85c2666"}	[{"id": "0cb14d82-caa5-4638-8e6b-949c0fad89e4", "name": "Internet", "subnet_id": "b4abcbcd-17ee-458a-a623-b307e5708a4e", "ip_address": "203.0.113.192", "mac_address": null}]	["207ed099-9a85-470b-9b64-55dff46d191f"]	[{"id": "2e9577ea-35c3-4500-8da1-2e76d421e5aa", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-23 23:00:49.165114+00	2025-10-23 23:00:49.183977+00
37ed97d2-8752-4cd4-9b0b-2e8e5d6d80e6	e861b2be-2cc4-4cb5-86ae-febd936afd2a	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "f9cdc868-8693-4806-8af2-3fb1145b9c5d"}	[{"id": "0f3ef0e6-e62d-4251-9e70-694e2e9aebd1", "name": "Remote Network", "subnet_id": "01f65759-d8ce-41c5-8f83-88bb7b369ede", "ip_address": "203.0.113.180", "mac_address": null}]	["e87f4509-0034-4b7e-853a-b80d50a18123"]	[{"id": "a6980625-39ae-46fd-a636-7e238545b89d", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-23 23:00:49.16512+00	2025-10-23 23:00:49.188699+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
e861b2be-2cc4-4cb5-86ae-febd936afd2a	My Network	2025-10-23 23:00:49.102457+00	2025-10-23 23:00:49.102462+00	t	4a445310-d759-4517-af20-17af3f62b8ff
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, service_definition, virtualization, source) FROM stdin;
2a8efd06-5bf7-4cac-b6d5-acc498a9a721	e861b2be-2cc4-4cb5-86ae-febd936afd2a	2025-10-23 23:00:49.165109+00	2025-10-23 23:00:49.165109+00	Cloudflare DNS	97d10182-da62-457f-ac70-137275c5718b	[{"id": "1ff736d4-9ef1-43c1-aa3e-914523d499a7", "type": "Layer4", "port_id": "6919de89-401d-4ce0-97de-ceee5106e6ec", "interface_id": "963b4331-37c4-422e-9214-881369f9ae8d"}]	"Dns Server"	null	{"type": "System"}
207ed099-9a85-470b-9b64-55dff46d191f	e861b2be-2cc4-4cb5-86ae-febd936afd2a	2025-10-23 23:00:49.165116+00	2025-10-23 23:00:49.165116+00	Google.com	e309d76c-1a29-4a3d-884d-d88010220a5b	[{"id": "76d97576-986f-4e9a-9642-94ebc85c2666", "type": "Layer4", "port_id": "2e9577ea-35c3-4500-8da1-2e76d421e5aa", "interface_id": "0cb14d82-caa5-4638-8e6b-949c0fad89e4"}]	"Web Service"	null	{"type": "System"}
e87f4509-0034-4b7e-853a-b80d50a18123	e861b2be-2cc4-4cb5-86ae-febd936afd2a	2025-10-23 23:00:49.165121+00	2025-10-23 23:00:49.165121+00	Mobile Device	37ed97d2-8752-4cd4-9b0b-2e8e5d6d80e6	[{"id": "f9cdc868-8693-4806-8af2-3fb1145b9c5d", "type": "Layer4", "port_id": "a6980625-39ae-46fd-a636-7e238545b89d", "interface_id": "0f3ef0e6-e62d-4251-9e70-694e2e9aebd1"}]	"Client"	null	{"type": "System"}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
b4abcbcd-17ee-458a-a623-b307e5708a4e	e861b2be-2cc4-4cb5-86ae-febd936afd2a	2025-10-23 23:00:49.165034+00	2025-10-23 23:00:49.165034+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
01f65759-d8ce-41c5-8f83-88bb7b369ede	e861b2be-2cc4-4cb5-86ae-febd936afd2a	2025-10-23 23:00:49.165042+00	2025-10-23 23:00:49.165042+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
4a445310-d759-4517-af20-17af3f62b8ff		2025-10-23 23:00:49.095149+00	2025-10-23 23:00:49.095191+00
\.


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: daemons daemons_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.daemons
    ADD CONSTRAINT daemons_pkey PRIMARY KEY (id);


--
-- Name: groups groups_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_pkey PRIMARY KEY (id);


--
-- Name: hosts hosts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_pkey PRIMARY KEY (id);


--
-- Name: networks networks_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.networks
    ADD CONSTRAINT networks_pkey PRIMARY KEY (id);


--
-- Name: services services_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_pkey PRIMARY KEY (id);


--
-- Name: subnets subnets_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_pkey PRIMARY KEY (id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: idx_daemon_host_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_daemon_host_id ON public.daemons USING btree (host_id);


--
-- Name: idx_daemons_network; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_daemons_network ON public.daemons USING btree (network_id);


--
-- Name: idx_groups_network; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_groups_network ON public.groups USING btree (network_id);


--
-- Name: idx_hosts_network; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_hosts_network ON public.hosts USING btree (network_id);


--
-- Name: idx_services_host_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_host_id ON public.services USING btree (host_id);


--
-- Name: idx_services_network; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_network ON public.services USING btree (network_id);


--
-- Name: idx_subnets_network; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnets_network ON public.subnets USING btree (network_id);


--
-- Name: daemons daemons_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.daemons
    ADD CONSTRAINT daemons_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- Name: groups groups_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- Name: hosts hosts_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- Name: networks networks_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.networks
    ADD CONSTRAINT networks_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: services services_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_host_id_fkey FOREIGN KEY (host_id) REFERENCES public.hosts(id) ON DELETE CASCADE;


--
-- Name: services services_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- Name: subnets subnets_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict vxk1ZN9GUoaaFGV9qtb4fAelSj1pfjUNVq2QKmbK1rg2x7oU7Bguko7UlltLpHG

