--
-- PostgreSQL database dump
--

\restrict ZsOmpM1WmC2ycByER9daNjttykgQpaI2QfGv4DyJnetp6dmBAbKSx3d0fmkIpnM

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
    source jsonb NOT NULL,
    color text NOT NULL
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
20251006215000	users	2025-10-26 14:52:42.320112+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	5700416
20251006215100	networks	2025-10-26 14:52:42.326574+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	4052917
20251006215151	create hosts	2025-10-26 14:52:42.331015+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	5378583
20251006215155	create subnets	2025-10-26 14:52:42.336719+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	5895833
20251006215201	create groups	2025-10-26 14:52:42.342928+00	t	\\x0a7032bf4d33a0baf020e905da865cde240e2a09dda2f62aa535b2c5d4b26b20be30a3286f1b5192bd94cd4a5dbb5bcd	5638917
20251006215204	create daemons	2025-10-26 14:52:42.34897+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	7000417
20251006215212	create services	2025-10-26 14:52:42.356252+00	t	\\xd5b07f82fc7c9da2782a364d46078d7d16b5c08df70cfbf02edcfe9b1b24ab6024ad159292aeea455f15cfd1f4740c1d	4022916
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
\.


--
-- Data for Name: groups; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.groups (id, network_id, name, description, group_type, created_at, updated_at, source, color) FROM stdin;
\.


--
-- Data for Name: hosts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.hosts (id, network_id, name, hostname, description, target, interfaces, services, ports, source, virtualization, created_at, updated_at) FROM stdin;
fb1dcb46-3fb8-44d9-9c53-b3091ed4391c	870cc7ce-9abc-45e1-b696-4f7a1dce6724	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "03e7e878-a8ac-48e7-83f6-04e61ca5d287"}	[{"id": "61e4f29c-c0e4-4803-931c-5079f683dc2a", "name": "Internet", "subnet_id": "4d5e3578-d735-41ae-a357-67135e099d98", "ip_address": "1.1.1.1", "mac_address": null}]	["6171ca25-25c5-490b-b5a1-996dcdd220c5"]	[{"id": "fd6165f0-20f7-48ac-b2e7-d6a504aa52d3", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-26 14:52:54.236468+00	2025-10-26 14:52:54.252778+00
26db5aab-a919-4a63-9781-93c79b53efed	870cc7ce-9abc-45e1-b696-4f7a1dce6724	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "d4ad6db9-f4d6-472d-a6e7-ad0209641457"}	[{"id": "426832cb-a769-4737-8943-b61c888ebc0f", "name": "Internet", "subnet_id": "4d5e3578-d735-41ae-a357-67135e099d98", "ip_address": "203.0.113.232", "mac_address": null}]	["96a7fb3f-e819-4787-8015-cfcd96a2b8fb"]	[{"id": "fd102522-7429-4f38-a20c-7548418401e0", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-26 14:52:54.236474+00	2025-10-26 14:52:54.257928+00
5cc8e222-746e-4dec-9612-0462a9b4671d	870cc7ce-9abc-45e1-b696-4f7a1dce6724	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "9db76846-80f5-4d0b-b6bb-3599aea50bef"}	[{"id": "b23278fa-c394-4ca7-b4b9-ccd842d5828b", "name": "Remote Network", "subnet_id": "44bf2602-2835-434c-9385-ea1e8595220b", "ip_address": "203.0.113.195", "mac_address": null}]	["2e4216ce-f95f-423b-82f8-4800e1b6200e"]	[{"id": "533ee696-3ab9-4bcc-a955-bf12bdda7f4d", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-26 14:52:54.236479+00	2025-10-26 14:52:54.263228+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
870cc7ce-9abc-45e1-b696-4f7a1dce6724	My Network	2025-10-26 14:52:54.186157+00	2025-10-26 14:52:54.186159+00	t	d590cd2e-5092-4601-b21e-909ccf85c88a
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, service_definition, virtualization, source) FROM stdin;
6171ca25-25c5-490b-b5a1-996dcdd220c5	870cc7ce-9abc-45e1-b696-4f7a1dce6724	2025-10-26 14:52:54.23647+00	2025-10-26 14:52:54.25176+00	Cloudflare DNS	fb1dcb46-3fb8-44d9-9c53-b3091ed4391c	[{"id": "03e7e878-a8ac-48e7-83f6-04e61ca5d287", "type": "Port", "port_id": "fd6165f0-20f7-48ac-b2e7-d6a504aa52d3", "interface_id": "61e4f29c-c0e4-4803-931c-5079f683dc2a"}]	"Dns Server"	null	{"type": "System"}
96a7fb3f-e819-4787-8015-cfcd96a2b8fb	870cc7ce-9abc-45e1-b696-4f7a1dce6724	2025-10-26 14:52:54.236475+00	2025-10-26 14:52:54.257382+00	Google.com	26db5aab-a919-4a63-9781-93c79b53efed	[{"id": "d4ad6db9-f4d6-472d-a6e7-ad0209641457", "type": "Port", "port_id": "fd102522-7429-4f38-a20c-7548418401e0", "interface_id": "426832cb-a769-4737-8943-b61c888ebc0f"}]	"Web Service"	null	{"type": "System"}
2e4216ce-f95f-423b-82f8-4800e1b6200e	870cc7ce-9abc-45e1-b696-4f7a1dce6724	2025-10-26 14:52:54.236481+00	2025-10-26 14:52:54.262613+00	Mobile Device	5cc8e222-746e-4dec-9612-0462a9b4671d	[{"id": "9db76846-80f5-4d0b-b6bb-3599aea50bef", "type": "Port", "port_id": "533ee696-3ab9-4bcc-a955-bf12bdda7f4d", "interface_id": "b23278fa-c394-4ca7-b4b9-ccd842d5828b"}]	"Client"	null	{"type": "System"}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
4d5e3578-d735-41ae-a357-67135e099d98	870cc7ce-9abc-45e1-b696-4f7a1dce6724	2025-10-26 14:52:54.236415+00	2025-10-26 14:52:54.236415+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
44bf2602-2835-434c-9385-ea1e8595220b	870cc7ce-9abc-45e1-b696-4f7a1dce6724	2025-10-26 14:52:54.236421+00	2025-10-26 14:52:54.236421+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
d590cd2e-5092-4601-b21e-909ccf85c88a		2025-10-26 14:52:54.181985+00	2025-10-26 14:52:54.181989+00
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

\unrestrict ZsOmpM1WmC2ycByER9daNjttykgQpaI2QfGv4DyJnetp6dmBAbKSx3d0fmkIpnM

