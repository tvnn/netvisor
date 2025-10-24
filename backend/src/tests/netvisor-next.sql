--
-- PostgreSQL database dump
--

\restrict zaq7JTPonhKXgLsSNRoQcQ3mIc3PCeeiSQSgRfjk49kDGobtXEYadn1O7TcgZaj

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
20251006215000	users	2025-10-24 02:00:14.391771+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	2218333
20251006215100	networks	2025-10-24 02:00:14.394644+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	2562125
20251006215151	create hosts	2025-10-24 02:00:14.397456+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	1815708
20251006215155	create subnets	2025-10-24 02:00:14.39948+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	1824541
20251006215201	create groups	2025-10-24 02:00:14.401502+00	t	\\x96cdc35b7ad03869a836d4a4fe8c3060d075c32edce248827903ceab5c4e41b0727300d6c5755e54973f3ada9e50293a	1535250
20251006215204	create daemons	2025-10-24 02:00:14.40323+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	1745500
20251006215212	create services	2025-10-24 02:00:14.405157+00	t	\\xd5b07f82fc7c9da2782a364d46078d7d16b5c08df70cfbf02edcfe9b1b24ab6024ad159292aeea455f15cfd1f4740c1d	2006084
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
f77197ea-d798-4a0b-b87d-cb1c519d41ad	c41b5a7a-3660-4ee6-a14f-623307e11bce	ca17ab9c-a39e-4167-a331-2697957f875f	"172.25.0.4"	60073	2025-10-24 02:05:06.879476+00	2025-10-24 02:06:06.898454+00
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
9fcc3868-d1ab-4428-a5d3-0728c5bcdb53	c41b5a7a-3660-4ee6-a14f-623307e11bce	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "6db255bd-5de1-48d5-9429-dba0bb679d58"}	[{"id": "4b4e0a19-fc3f-4553-8c82-079f77862616", "name": "Internet", "subnet_id": "162caa3f-c7e2-4fbe-b0f6-9bba494bc843", "ip_address": "1.1.1.1", "mac_address": null}]	["9af40ed9-23de-4eee-b790-d1adf0041808"]	[{"id": "0994284a-d51f-49b9-8d48-d38ec1d89292", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-24 02:00:14.451163+00	2025-10-24 02:00:14.461826+00
f326fd5d-46d7-46fb-afab-554ceaf81ce5	c41b5a7a-3660-4ee6-a14f-623307e11bce	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "3ecc0fe4-89e1-425a-afc3-4ef8effb9ee5"}	[{"id": "28a8625a-4b3b-43e7-9da7-b3dcf5b08d7b", "name": "Internet", "subnet_id": "162caa3f-c7e2-4fbe-b0f6-9bba494bc843", "ip_address": "203.0.113.154", "mac_address": null}]	["145305de-264e-40ce-be60-7de4001395fc"]	[{"id": "a0a6dc9a-9b02-49d0-8fd9-baeaa7c5b342", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-24 02:00:14.451184+00	2025-10-24 02:00:14.46526+00
3f5a8d04-cd66-464e-9f02-ae02d2bdc15a	c41b5a7a-3660-4ee6-a14f-623307e11bce	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "7dbe95d7-a1bb-4ebb-8562-146e44ba6f4e"}	[{"id": "2e909195-c751-4a3c-9161-dc8c74ad60ac", "name": "Remote Network", "subnet_id": "347cdc49-684d-41ea-a248-d10c1d8b250f", "ip_address": "203.0.113.119", "mac_address": null}]	["bbcb1b62-08bb-4300-b2a3-964528d880a0"]	[{"id": "dfc312e1-146f-4790-8963-bc81585de77d", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-24 02:00:14.451186+00	2025-10-24 02:00:14.467971+00
a12b9503-4721-4946-b888-becfe9b48596	c41b5a7a-3660-4ee6-a14f-623307e11bce	NetVisor Server API	netvisor-server-1.netvisor_netvisor-dev	Discovered host	{"type": "Hostname"}	[{"id": "8326084b-6073-4573-a6ab-ae510bbe6cfa", "name": null, "subnet_id": "08818351-e15b-42f5-b010-9899a1977aed", "ip_address": "172.25.0.3", "mac_address": "32:81:E6:F1:20:41"}]	["a2555046-867b-49d3-b763-4d95f5df501f"]	[{"id": "4af90da0-67b5-4101-97b6-2be47efdf1f8", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-24T02:05:15.866873135Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}]}	null	2025-10-24 02:05:15.866877+00	2025-10-24 02:05:33.948595+00
ca17ab9c-a39e-4167-a331-2697957f875f	c41b5a7a-3660-4ee6-a14f-623307e11bce	a561d2751937	a561d2751937	NetVisor daemon	{"type": "Hostname"}	[{"id": "b85e3f7b-df73-4584-8788-d6a202506d02", "name": "eth0", "subnet_id": "08818351-e15b-42f5-b010-9899a1977aed", "ip_address": "172.25.0.4", "mac_address": "16:58:BA:17:12:4C"}]	["c1af14bf-728d-4ae5-a377-c678c642dc20", "616988e3-cfae-4a50-b31b-58d7ee001212"]	[{"id": "bd6bc4fc-2d8c-4ac2-b477-0ab360b1f227", "type": "Custom", "number": 60073, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-24T02:05:15.863389552Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}, {"date": "2025-10-24T02:05:06.865697798Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "SelfReport"}]}	null	2025-10-24 02:05:06.8657+00	2025-10-24 02:05:15.877092+00
3839fc92-954b-415f-8e65-f53a5f605a1a	c41b5a7a-3660-4ee6-a14f-623307e11bce	Home Assistant	homeassistant-discovery.netvisor_netvisor-dev	Discovered host	{"type": "Hostname"}	[{"id": "ad8c9e0b-4ac4-4a5b-aba1-aeed7994ca8d", "name": null, "subnet_id": "08818351-e15b-42f5-b010-9899a1977aed", "ip_address": "172.25.0.5", "mac_address": "02:2B:88:49:AD:B5"}]	["0f677168-db20-4e6b-b2f5-4a3f2d7e5d10"]	[{"id": "deb8dd89-302d-47e4-8afa-21e314e85184", "type": "Custom", "number": 8123, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-24T02:05:33.933862171Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}]}	null	2025-10-24 02:05:33.933865+00	2025-10-24 02:06:19.419678+00
224dab50-9a06-4670-b825-191af2b5132b	c41b5a7a-3660-4ee6-a14f-623307e11bce	NetVisor Server API	\N	Discovered host	{"type": "None"}	[{"id": "7aa5815a-92d4-4c97-861c-0dc55a3de2f1", "name": null, "subnet_id": "08818351-e15b-42f5-b010-9899a1977aed", "ip_address": "172.25.0.1", "mac_address": "12:AA:41:D3:53:B2"}]	["82c5c4d2-101c-455e-9ed6-286d0b7395ab", "a2b87532-daec-48c9-a2d5-48d21838b9fb", "e7d59c64-f791-48ab-9402-536ca17ff91f"]	[{"id": "12464b44-8a20-4170-8d26-8d35d399fc90", "type": "Custom", "number": 60072, "protocol": "Tcp"}, {"id": "4987e834-571a-497e-aa82-bfc71e019371", "type": "Custom", "number": 8123, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-24T02:05:54.809137375Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}]}	null	2025-10-24 02:05:54.809139+00	2025-10-24 02:06:19.478845+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
c41b5a7a-3660-4ee6-a14f-623307e11bce	My Network	2025-10-24 02:00:14.409278+00	2025-10-24 02:00:14.409279+00	t	fd4513d5-6cf2-4a78-b5d2-720d3c217ec5
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, service_definition, virtualization, source) FROM stdin;
9af40ed9-23de-4eee-b790-d1adf0041808	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:00:14.451179+00	2025-10-24 02:00:14.451179+00	Cloudflare DNS	9fcc3868-d1ab-4428-a5d3-0728c5bcdb53	[{"id": "6db255bd-5de1-48d5-9429-dba0bb679d58", "type": "Layer4", "port_id": "0994284a-d51f-49b9-8d48-d38ec1d89292", "interface_id": "4b4e0a19-fc3f-4553-8c82-079f77862616"}]	"Dns Server"	null	{"type": "System"}
145305de-264e-40ce-be60-7de4001395fc	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:00:14.451184+00	2025-10-24 02:00:14.451184+00	Google.com	f326fd5d-46d7-46fb-afab-554ceaf81ce5	[{"id": "3ecc0fe4-89e1-425a-afc3-4ef8effb9ee5", "type": "Layer4", "port_id": "a0a6dc9a-9b02-49d0-8fd9-baeaa7c5b342", "interface_id": "28a8625a-4b3b-43e7-9da7-b3dcf5b08d7b"}]	"Web Service"	null	{"type": "System"}
bbcb1b62-08bb-4300-b2a3-964528d880a0	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:00:14.451187+00	2025-10-24 02:00:14.451187+00	Mobile Device	3f5a8d04-cd66-464e-9f02-ae02d2bdc15a	[{"id": "7dbe95d7-a1bb-4ebb-8562-146e44ba6f4e", "type": "Layer4", "port_id": "dfc312e1-146f-4790-8963-bc81585de77d", "interface_id": "2e909195-c751-4a3c-9161-dc8c74ad60ac"}]	"Client"	null	{"type": "System"}
c1af14bf-728d-4ae5-a377-c678c642dc20	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:05:06.865715+00	2025-10-24 02:05:06.865715+00	NetVisor Daemon API	ca17ab9c-a39e-4167-a331-2697957f875f	[{"id": "c90e81e5-404b-42e8-8550-297a5bf1d6ba", "type": "Layer4", "port_id": "bd6bc4fc-2d8c-4ac2-b477-0ab360b1f227", "interface_id": "b85e3f7b-df73-4584-8788-d6a202506d02"}]	"NetVisor Daemon API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Updated match data on 2025-10-24 02:05:15.864363635 UTC", [{"data": "Response from http://172.25.0.4:60073/api/health contained \\"netvisor\\"", "type": "reason"}, {"data": "NetVisor Daemon self-report", "type": "reason"}]], "type": "container"}, "confidence": "Certain"}, "metadata": [{"date": "2025-10-24T02:05:15.864363635Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}, {"date": "2025-10-24T02:05:06.865714089Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "SelfReport"}]}
a2555046-867b-49d3-b763-4d95f5df501f	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:05:27.600132+00	2025-10-24 02:05:27.600132+00	NetVisor Server API	a12b9503-4721-4946-b888-becfe9b48596	[{"id": "5931542b-100c-4f9d-9e7b-135bd825357a", "type": "Layer4", "port_id": "4af90da0-67b5-4101-97b6-2be47efdf1f8", "interface_id": "8326084b-6073-4573-a6ab-ae510bbe6cfa"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.3:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-24T02:05:27.599978043Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}]}
82c5c4d2-101c-455e-9ed6-286d0b7395ab	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:06:08.806324+00	2025-10-24 02:06:08.806324+00	NetVisor Server API	224dab50-9a06-4670-b825-191af2b5132b	[{"id": "ef26efb9-c58b-4651-b55a-b0d035c4a64c", "type": "Layer4", "port_id": "12464b44-8a20-4170-8d26-8d35d399fc90", "interface_id": "7aa5815a-92d4-4c97-861c-0dc55a3de2f1"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-24T02:06:08.806086215Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}]}
a2b87532-daec-48c9-a2d5-48d21838b9fb	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:06:13.631016+00	2025-10-24 02:06:13.631016+00	Home Assistant	224dab50-9a06-4670-b825-191af2b5132b	[{"id": "2724930f-1f0e-4642-9a7c-9ff023f2ae0c", "type": "Layer4", "port_id": "4987e834-571a-497e-aa82-bfc71e019371", "interface_id": "7aa5815a-92d4-4c97-861c-0dc55a3de2f1"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-24T02:06:13.630875593Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}]}
0f677168-db20-4e6b-b2f5-4a3f2d7e5d10	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:05:52.443266+00	2025-10-24 02:05:52.443266+00	Home Assistant	3839fc92-954b-415f-8e65-f53a5f605a1a	[{"id": "e7108152-80ff-453d-b47d-1b4a20f4350c", "type": "Layer4", "port_id": "deb8dd89-302d-47e4-8afa-21e314e85184", "interface_id": "ad8c9e0b-4ac4-4a5b-aba1-aeed7994ca8d"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.5:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-24T02:05:52.442893638Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}]}
e7d59c64-f791-48ab-9402-536ca17ff91f	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:06:19.273859+00	2025-10-24 02:06:19.273859+00	Gateway	224dab50-9a06-4670-b825-191af2b5132b	[{"id": "beb3c36f-2b44-4a21-9788-4a809a619bcb", "type": "Layer3", "interface_id": "7aa5815a-92d4-4c97-861c-0dc55a3de2f1"}]	"Gateway"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Match confidence for generic services is N/A", [{"data": "Host IP address is in routing table of daemon f77197ea-d798-4a0b-b87d-cb1c519d41ad", "type": "reason"}]], "type": "container"}, "confidence": "NotApplicable"}, "metadata": [{"date": "2025-10-24T02:06:19.273291720Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "Network"}]}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
162caa3f-c7e2-4fbe-b0f6-9bba494bc843	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:00:14.451074+00	2025-10-24 02:00:14.451074+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
347cdc49-684d-41ea-a248-d10c1d8b250f	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:00:14.451084+00	2025-10-24 02:00:14.451084+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
08818351-e15b-42f5-b010-9899a1977aed	c41b5a7a-3660-4ee6-a14f-623307e11bce	2025-10-24 02:05:06.851531+00	2025-10-24 02:05:06.851531+00	"172.25.0.0/28"	172.25.0.0/28	\N	"Lan"	{"type": "Discovery", "metadata": [{"date": "2025-10-24T02:05:06.851486298Z", "daemon_id": "f77197ea-d798-4a0b-b87d-cb1c519d41ad", "discovery_type": "SelfReport"}]}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
fd4513d5-6cf2-4a78-b5d2-720d3c217ec5	Default Username	2025-10-24 02:00:14.408722+00	2025-10-24 02:00:14.40873+00
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

\unrestrict zaq7JTPonhKXgLsSNRoQcQ3mIc3PCeeiSQSgRfjk49kDGobtXEYadn1O7TcgZaj

