--
-- PostgreSQL database dump
--

\restrict Ib1NtxyD07t3lc3qAB4mMtlTVcApoWFF5bCg1D39wYMEI3CV76VuvhztNPEDPTP

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
    group_type jsonb NOT NULL,
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
    is_gateway boolean,
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
20251006215000	users	2025-10-24 22:10:30.896288+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	4037875
20251006215100	networks	2025-10-24 22:10:30.901448+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	3364917
20251006215151	create hosts	2025-10-24 22:10:30.905183+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	2663542
20251006215155	create subnets	2025-10-24 22:10:30.90819+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	3307167
20251006215201	create groups	2025-10-24 22:10:30.911816+00	t	\\x28b2ad8fa9dd6a96e8798db9c9c5884c91805f5265111ff9dcd3be0222fcb4370a9ae5829600c0281c81540a0c351d90	2471708
20251006215204	create daemons	2025-10-24 22:10:30.914622+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	2858708
20251006215212	create services	2025-10-24 22:10:30.917785+00	t	\\xd96da6bc018518b42c9bcab782539ea9b8c0324448b2889ef3438282ac019131b6493a54f048957613d779da672cd4a8	2770958
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
\.


--
-- Data for Name: groups; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.groups (id, network_id, name, description, group_type, created_at, updated_at, source) FROM stdin;
\.


--
-- Data for Name: hosts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.hosts (id, network_id, name, hostname, description, target, interfaces, services, ports, source, virtualization, created_at, updated_at) FROM stdin;
87dd109b-b2d1-4307-9d6c-87d8cd2a8d00	20f4582c-11b4-4017-b664-1186e563e991	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "80b6399b-a332-4022-87a8-d12f105baccb"}	[{"id": "518c0003-a96b-4532-a47e-b01b88be80a9", "name": "Internet", "subnet_id": "1ac3d783-c575-47e0-a815-e1cdd389cf22", "ip_address": "1.1.1.1", "mac_address": null}]	["6cb42611-b0f4-4b3c-81c8-5138c5b77f69"]	[{"id": "cfc5aadf-3eab-4c63-b1bc-0bb9d64afe62", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-24 22:10:36.394772+00	2025-10-24 22:10:36.413681+00
2d4f3a00-694d-4923-a62d-b3b7ac194b6f	20f4582c-11b4-4017-b664-1186e563e991	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "0eacc127-ea33-4ff4-ae3f-d5920353daec"}	[{"id": "ceee803a-e084-4077-9a7d-87f309d213b5", "name": "Internet", "subnet_id": "1ac3d783-c575-47e0-a815-e1cdd389cf22", "ip_address": "203.0.113.76", "mac_address": null}]	["20deef8f-dd02-49ef-94cd-2ca71c94dffd"]	[{"id": "06525e99-a360-4a03-9d84-c601fe620c30", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-24 22:10:36.394777+00	2025-10-24 22:10:36.419243+00
a586fc77-9459-417e-ac31-f2d9ba93804c	20f4582c-11b4-4017-b664-1186e563e991	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "54e25798-8622-4ca1-b355-3538c010b4e0"}	[{"id": "6e01e40c-9bc2-4d6a-906b-e7c0fc35c952", "name": "Remote Network", "subnet_id": "bd9befca-2b0f-4a7a-a8a9-4d80fff49701", "ip_address": "203.0.113.125", "mac_address": null}]	["8d633b32-5959-4235-97f0-ff36ac6d1046"]	[{"id": "fccf59ca-29ca-4f84-a860-9bfafcde58da", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-24 22:10:36.394782+00	2025-10-24 22:10:36.42436+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
20f4582c-11b4-4017-b664-1186e563e991	My Network	2025-10-24 22:10:36.347986+00	2025-10-24 22:10:36.347988+00	t	433e7f4d-dab1-4611-a2ed-5a46aca8fccc
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, is_gateway, service_definition, virtualization, source) FROM stdin;
6cb42611-b0f4-4b3c-81c8-5138c5b77f69	20f4582c-11b4-4017-b664-1186e563e991	2025-10-24 22:10:36.394773+00	2025-10-24 22:10:36.412608+00	Cloudflare DNS	87dd109b-b2d1-4307-9d6c-87d8cd2a8d00	[{"id": "80b6399b-a332-4022-87a8-d12f105baccb", "type": "Port", "port_id": "cfc5aadf-3eab-4c63-b1bc-0bb9d64afe62", "interface_id": "518c0003-a96b-4532-a47e-b01b88be80a9"}]	f	"Dns Server"	null	{"type": "System"}
20deef8f-dd02-49ef-94cd-2ca71c94dffd	20f4582c-11b4-4017-b664-1186e563e991	2025-10-24 22:10:36.394778+00	2025-10-24 22:10:36.418625+00	Google.com	2d4f3a00-694d-4923-a62d-b3b7ac194b6f	[{"id": "0eacc127-ea33-4ff4-ae3f-d5920353daec", "type": "Port", "port_id": "06525e99-a360-4a03-9d84-c601fe620c30", "interface_id": "ceee803a-e084-4077-9a7d-87f309d213b5"}]	f	"Web Service"	null	{"type": "System"}
8d633b32-5959-4235-97f0-ff36ac6d1046	20f4582c-11b4-4017-b664-1186e563e991	2025-10-24 22:10:36.394783+00	2025-10-24 22:10:36.42377+00	Mobile Device	a586fc77-9459-417e-ac31-f2d9ba93804c	[{"id": "54e25798-8622-4ca1-b355-3538c010b4e0", "type": "Port", "port_id": "fccf59ca-29ca-4f84-a860-9bfafcde58da", "interface_id": "6e01e40c-9bc2-4d6a-906b-e7c0fc35c952"}]	f	"Client"	null	{"type": "System"}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
1ac3d783-c575-47e0-a815-e1cdd389cf22	20f4582c-11b4-4017-b664-1186e563e991	2025-10-24 22:10:36.394695+00	2025-10-24 22:10:36.394695+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
bd9befca-2b0f-4a7a-a8a9-4d80fff49701	20f4582c-11b4-4017-b664-1186e563e991	2025-10-24 22:10:36.394703+00	2025-10-24 22:10:36.394703+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
433e7f4d-dab1-4611-a2ed-5a46aca8fccc		2025-10-24 22:10:36.345537+00	2025-10-24 22:10:36.345541+00
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

\unrestrict Ib1NtxyD07t3lc3qAB4mMtlTVcApoWFF5bCg1D39wYMEI3CV76VuvhztNPEDPTP

