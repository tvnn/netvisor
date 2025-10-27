--
-- PostgreSQL database dump
--

\restrict RT2w42k1bshFyiwnvk0d0kc8JinicFQ0ESomJDlAYdDTxru0Fgfro0I7BzK1v1a

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

ALTER TABLE IF EXISTS ONLY public.subnets DROP CONSTRAINT IF EXISTS subnets_network_id_fkey;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_network_id_fkey;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_host_id_fkey;
ALTER TABLE IF EXISTS ONLY public.networks DROP CONSTRAINT IF EXISTS networks_user_id_fkey;
ALTER TABLE IF EXISTS ONLY public.hosts DROP CONSTRAINT IF EXISTS hosts_network_id_fkey;
ALTER TABLE IF EXISTS ONLY public.groups DROP CONSTRAINT IF EXISTS groups_network_id_fkey;
ALTER TABLE IF EXISTS ONLY public.daemons DROP CONSTRAINT IF EXISTS daemons_network_id_fkey;
DROP INDEX IF EXISTS public.idx_subnets_network;
DROP INDEX IF EXISTS public.idx_services_network;
DROP INDEX IF EXISTS public.idx_services_host_id;
DROP INDEX IF EXISTS public.idx_hosts_network;
DROP INDEX IF EXISTS public.idx_groups_network;
DROP INDEX IF EXISTS public.idx_daemons_network;
DROP INDEX IF EXISTS public.idx_daemon_host_id;
ALTER TABLE IF EXISTS ONLY public.users DROP CONSTRAINT IF EXISTS users_pkey;
ALTER TABLE IF EXISTS ONLY public.subnets DROP CONSTRAINT IF EXISTS subnets_pkey;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_pkey;
ALTER TABLE IF EXISTS ONLY public.networks DROP CONSTRAINT IF EXISTS networks_pkey;
ALTER TABLE IF EXISTS ONLY public.hosts DROP CONSTRAINT IF EXISTS hosts_pkey;
ALTER TABLE IF EXISTS ONLY public.groups DROP CONSTRAINT IF EXISTS groups_pkey;
ALTER TABLE IF EXISTS ONLY public.daemons DROP CONSTRAINT IF EXISTS daemons_pkey;
ALTER TABLE IF EXISTS ONLY public._sqlx_migrations DROP CONSTRAINT IF EXISTS _sqlx_migrations_pkey;
DROP TABLE IF EXISTS public.users;
DROP TABLE IF EXISTS public.subnets;
DROP TABLE IF EXISTS public.services;
DROP TABLE IF EXISTS public.networks;
DROP TABLE IF EXISTS public.hosts;
DROP TABLE IF EXISTS public.groups;
DROP TABLE IF EXISTS public.daemons;
DROP TABLE IF EXISTS public._sqlx_migrations;
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
20251006215000	users	2025-10-26 15:30:12.299778+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	1952416
20251006215100	networks	2025-10-26 15:30:12.302405+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	1808083
20251006215151	create hosts	2025-10-26 15:30:12.304419+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	1458667
20251006215155	create subnets	2025-10-26 15:30:12.306092+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	1662625
20251006215201	create groups	2025-10-26 15:30:12.307936+00	t	\\x0a7032bf4d33a0baf020e905da865cde240e2a09dda2f62aa535b2c5d4b26b20be30a3286f1b5192bd94cd4a5dbb5bcd	1352708
20251006215204	create daemons	2025-10-26 15:30:12.309467+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	1679834
20251006215212	create services	2025-10-26 15:30:12.311481+00	t	\\xd5b07f82fc7c9da2782a364d46078d7d16b5c08df70cfbf02edcfe9b1b24ab6024ad159292aeea455f15cfd1f4740c1d	1807708
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
c86f5832-9a8b-4645-8237-8aa9125b4724	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	d085edd0-ac98-46c6-9ad3-052fac59a18e	"172.25.0.4"	60073	2025-10-26 15:31:19.796334+00	2025-10-26 15:31:49.808043+00
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
171e9d9c-b3d3-49a8-9c2d-3deb406a0898	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "63dae8da-5556-4609-b5d9-3fb18410b212"}	[{"id": "d5ace9b7-e571-4ef6-9c72-ac3288ff1e63", "name": "Internet", "subnet_id": "d9822440-72f2-45c8-b462-c338b9c34a9c", "ip_address": "1.1.1.1", "mac_address": null}]	["8573260c-b922-4b81-bd8c-f7cf28f281bc"]	[{"id": "f03f3f88-d0dd-45c7-82c5-e215e65b94e7", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-26 15:30:12.348246+00	2025-10-26 15:30:12.359139+00
7ce7da72-1e50-496c-9163-56baef9bde69	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "ead1f569-b1df-4477-886b-e837bd667271"}	[{"id": "cb77cd25-2ac7-4765-b919-74d20662b310", "name": "Internet", "subnet_id": "d9822440-72f2-45c8-b462-c338b9c34a9c", "ip_address": "203.0.113.236", "mac_address": null}]	["24665d18-9b18-4c1b-a528-7fb7e292d67c"]	[{"id": "0d58857e-0f6a-48cd-822e-07247ae1d5df", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-26 15:30:12.348251+00	2025-10-26 15:30:12.362173+00
12a49a30-2d87-4cb5-acbe-c78d05521b76	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "de395073-be42-462f-bab1-320ecb37eef1"}	[{"id": "e7ec6324-4559-4f5b-985f-1f2cb849e32a", "name": "Remote Network", "subnet_id": "9ba3ad10-f317-445e-afb7-2b91a016ee5e", "ip_address": "203.0.113.110", "mac_address": null}]	["5c3159e2-133f-4e54-afdf-02b6a38ded01"]	[{"id": "9fbfb5b5-1eaa-43ad-9835-90568281bb98", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-26 15:30:12.348253+00	2025-10-26 15:30:12.364715+00
5c6c2369-e325-45b1-a80f-2cc050dced87	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	NetVisor Server API	netvisor-server-1.netvisor_netvisor-dev	\N	{"type": "Hostname"}	[{"id": "4b220a66-9c4b-44b2-b748-b682e2153446", "name": null, "subnet_id": "5ed71784-c0e2-436e-acfc-bca6442579fb", "ip_address": "172.25.0.3", "mac_address": "56:A9:A9:C6:B4:A8"}]	["a7680d70-9d77-4996-a24f-ad32697439c9"]	[{"id": "4910cc57-ae68-4fc7-b1f7-bdbff8afd43d", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-26T15:31:29.078874679Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "Network"}]}	null	2025-10-26 15:31:29.078876+00	2025-10-26 15:31:38.088773+00
d085edd0-ac98-46c6-9ad3-052fac59a18e	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	d36aebfa5d22	d36aebfa5d22	NetVisor daemon	{"type": "Hostname"}	[{"id": "153f4e98-68e7-4fe3-97a9-807c2de3441a", "name": "eth0", "subnet_id": "5ed71784-c0e2-436e-acfc-bca6442579fb", "ip_address": "172.25.0.4", "mac_address": "7E:B7:40:AD:36:F9"}]	["a06b3b51-7198-4532-ae32-af295d233dc4", "d1fbba52-9920-44ec-b22e-447f466465d6"]	[{"id": "6d8e4a16-d025-4a7c-8368-270f5165bedc", "type": "Custom", "number": 60073, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-26T15:31:29.076360679Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "Network"}, {"date": "2025-10-26T15:31:19.791154424Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "SelfReport"}]}	null	2025-10-26 15:31:19.791156+00	2025-10-26 15:31:29.089696+00
5ff4cffa-9681-4c9a-ac1f-ff47c48048e1	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	Home Assistant	homeassistant-discovery.netvisor_netvisor-dev	\N	{"type": "Hostname"}	[{"id": "99c05cf5-4ac5-43d1-b607-c9de0f5dd854", "name": null, "subnet_id": "5ed71784-c0e2-436e-acfc-bca6442579fb", "ip_address": "172.25.0.5", "mac_address": "B2:62:D6:4F:51:49"}]	["889346ee-ab06-4f93-9660-679bc2bceca7"]	[{"id": "e09459a8-141e-403a-a5d9-a709c3f45237", "type": "Custom", "number": 8123, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-26T15:31:38.084950210Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "Network"}]}	null	2025-10-26 15:31:38.084951+00	2025-10-26 15:31:56.66306+00
b3b28176-dc68-447c-b4c5-485ab90014ab	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	Home Assistant	\N	\N	{"type": "None"}	[{"id": "1d0070f0-2d17-4188-b5e0-1f3734a2d0dc", "name": null, "subnet_id": "5ed71784-c0e2-436e-acfc-bca6442579fb", "ip_address": "172.25.0.1", "mac_address": "26:12:2E:ED:D0:07"}]	["39385a30-219a-4882-9d89-759a1c1fa4b4", "3dacc8e4-8b96-41f1-b88d-82a294874dbd"]	[{"id": "defb8f76-aa1e-4ee5-b3ca-04d6169cd4a6", "type": "Custom", "number": 8123, "protocol": "Tcp"}, {"id": "d5dddfeb-e0d7-48b7-8db4-621b26916980", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-26T15:31:47.701761340Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "Network"}]}	null	2025-10-26 15:31:47.701766+00	2025-10-26 15:31:56.691947+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	My Network	2025-10-26 15:30:12.315217+00	2025-10-26 15:30:12.315218+00	t	297c6fee-62e2-41eb-8305-f5ebe3a8de44
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, service_definition, virtualization, source) FROM stdin;
8573260c-b922-4b81-bd8c-f7cf28f281bc	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:30:12.348247+00	2025-10-26 15:30:12.358453+00	Cloudflare DNS	171e9d9c-b3d3-49a8-9c2d-3deb406a0898	[{"id": "63dae8da-5556-4609-b5d9-3fb18410b212", "type": "Port", "port_id": "f03f3f88-d0dd-45c7-82c5-e215e65b94e7", "interface_id": "d5ace9b7-e571-4ef6-9c72-ac3288ff1e63"}]	"Dns Server"	null	{"type": "System"}
24665d18-9b18-4c1b-a528-7fb7e292d67c	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:30:12.348251+00	2025-10-26 15:30:12.361827+00	Google.com	7ce7da72-1e50-496c-9163-56baef9bde69	[{"id": "ead1f569-b1df-4477-886b-e837bd667271", "type": "Port", "port_id": "0d58857e-0f6a-48cd-822e-07247ae1d5df", "interface_id": "cb77cd25-2ac7-4765-b919-74d20662b310"}]	"Web Service"	null	{"type": "System"}
5c3159e2-133f-4e54-afdf-02b6a38ded01	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:30:12.348254+00	2025-10-26 15:30:12.364395+00	Mobile Device	12a49a30-2d87-4cb5-acbe-c78d05521b76	[{"id": "de395073-be42-462f-bab1-320ecb37eef1", "type": "Port", "port_id": "9fbfb5b5-1eaa-43ad-9835-90568281bb98", "interface_id": "e7ec6324-4559-4f5b-985f-1f2cb849e32a"}]	"Client"	null	{"type": "System"}
889346ee-ab06-4f93-9660-679bc2bceca7	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:31:40.327002+00	2025-10-26 15:31:56.662679+00	Home Assistant	5ff4cffa-9681-4c9a-ac1f-ff47c48048e1	[{"id": "5df7832e-2bae-4fa9-a6c0-934ad1ac2499", "type": "Port", "port_id": "e09459a8-141e-403a-a5d9-a709c3f45237", "interface_id": "99c05cf5-4ac5-43d1-b607-c9de0f5dd854"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.5:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-26T15:31:40.326993628Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "Network"}]}
a06b3b51-7198-4532-ae32-af295d233dc4	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:31:19.791162+00	2025-10-26 15:31:29.088831+00	NetVisor Daemon API	d085edd0-ac98-46c6-9ad3-052fac59a18e	[{"id": "65cd07ab-b6e3-47d2-bf65-0e64a92d46e1", "type": "Port", "port_id": "6d8e4a16-d025-4a7c-8368-270f5165bedc", "interface_id": "153f4e98-68e7-4fe3-97a9-807c2de3441a"}]	"NetVisor Daemon API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Updated match data on 2025-10-26 15:31:29.076902054 UTC", [{"data": "Response from http://172.25.0.4:60073/api/health contained \\"netvisor\\"", "type": "reason"}, {"data": "NetVisor Daemon self-report", "type": "reason"}]], "type": "container"}, "confidence": "Certain"}, "metadata": [{"date": "2025-10-26T15:31:29.076902054Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "Network"}, {"date": "2025-10-26T15:31:19.791162341Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "SelfReport"}]}
a7680d70-9d77-4996-a24f-ad32697439c9	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:31:35.395593+00	2025-10-26 15:31:38.08839+00	NetVisor Server API	5c6c2369-e325-45b1-a80f-2cc050dced87	[{"id": "bc8d2ea4-3662-4ba9-9213-c4b5bb4db550", "type": "Port", "port_id": "4910cc57-ae68-4fc7-b1f7-bdbff8afd43d", "interface_id": "4b220a66-9c4b-44b2-b748-b682e2153446"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.3:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-26T15:31:35.395579792Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "Network"}]}
39385a30-219a-4882-9d89-759a1c1fa4b4	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:31:49.936509+00	2025-10-26 15:31:56.691439+00	Home Assistant	b3b28176-dc68-447c-b4c5-485ab90014ab	[{"id": "0123dd32-edb8-4b0f-99e9-3c4c8c374f47", "type": "Port", "port_id": "defb8f76-aa1e-4ee5-b3ca-04d6169cd4a6", "interface_id": "1d0070f0-2d17-4188-b5e0-1f3734a2d0dc"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-26T15:31:49.936501008Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "Network"}]}
3dacc8e4-8b96-41f1-b88d-82a294874dbd	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:31:53.924824+00	2025-10-26 15:31:56.691546+00	NetVisor Server API	b3b28176-dc68-447c-b4c5-485ab90014ab	[{"id": "ad023a2e-bd92-4350-8b03-14a10094b248", "type": "Port", "port_id": "d5dddfeb-e0d7-48b7-8db4-621b26916980", "interface_id": "1d0070f0-2d17-4188-b5e0-1f3734a2d0dc"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-26T15:31:53.924815885Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "Network"}]}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
d9822440-72f2-45c8-b462-c338b9c34a9c	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:30:12.348148+00	2025-10-26 15:30:12.348148+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
9ba3ad10-f317-445e-afb7-2b91a016ee5e	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:30:12.348156+00	2025-10-26 15:30:12.348156+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
5ed71784-c0e2-436e-acfc-bca6442579fb	ceb4cab2-8d62-4a1d-a444-f5a79c8ab19f	2025-10-26 15:31:19.681027+00	2025-10-26 15:31:19.681027+00	"172.25.0.0/28"	172.25.0.0/28	\N	"Lan"	{"type": "Discovery", "metadata": [{"date": "2025-10-26T15:31:19.681026299Z", "daemon_id": "c86f5832-9a8b-4645-8237-8aa9125b4724", "discovery_type": "SelfReport"}]}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
297c6fee-62e2-41eb-8305-f5ebe3a8de44	Default Username	2025-10-26 15:30:12.314672+00	2025-10-26 15:30:12.314675+00
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

\unrestrict RT2w42k1bshFyiwnvk0d0kc8JinicFQ0ESomJDlAYdDTxru0Fgfro0I7BzK1v1a

