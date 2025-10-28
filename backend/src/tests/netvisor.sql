--
-- PostgreSQL database dump
--

\restrict L5177E3frdF5KhlGBkNb9dwbaMUJa1b8cMs1f5hiI854xf9gdvhWcwxtaC82vdg

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
20251006215000	users	2025-10-28 08:15:22.36191+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	3126750
20251006215100	networks	2025-10-28 08:15:22.366017+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	2732542
20251006215151	create hosts	2025-10-28 08:15:22.368971+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	1580625
20251006215155	create subnets	2025-10-28 08:15:22.370739+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	1766625
20251006215201	create groups	2025-10-28 08:15:22.3727+00	t	\\x0a7032bf4d33a0baf020e905da865cde240e2a09dda2f62aa535b2c5d4b26b20be30a3286f1b5192bd94cd4a5dbb5bcd	1569333
20251006215204	create daemons	2025-10-28 08:15:22.374443+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	1794041
20251006215212	create services	2025-10-28 08:15:22.376432+00	t	\\xd5b07f82fc7c9da2782a364d46078d7d16b5c08df70cfbf02edcfe9b1b24ab6024ad159292aeea455f15cfd1f4740c1d	1817792
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
d001b4db-368a-485d-81f7-045a854fe593	00145ea8-e012-4015-83ac-00e3ed3e7a0e	868f4a73-4080-49f6-adf3-5436bc6d1e5a	"172.25.0.4"	60073	2025-10-28 08:15:22.499246+00	2025-10-28 08:15:22.499244+00
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
e70ef2fc-1d4d-45bf-8fb8-c39b00608bd7	00145ea8-e012-4015-83ac-00e3ed3e7a0e	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "a7573bba-fa22-4437-9181-f41104a25793"}	[{"id": "a112de46-6d90-4fa9-b617-15bd36add619", "name": "Internet", "subnet_id": "e1263fb7-0f52-4ded-8715-fb2a94f9bf4c", "ip_address": "1.1.1.1", "mac_address": null}]	["28b27aac-26e5-45c1-ba72-de0934250f66"]	[{"id": "69cfac47-1423-4026-ad79-61d02a0fe227", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-28 08:15:22.419901+00	2025-10-28 08:15:22.433435+00
1a5a585d-df60-469a-8af8-ac3dad035378	00145ea8-e012-4015-83ac-00e3ed3e7a0e	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "e7dfc1e2-7582-4aa3-a859-1a6a4b39269e"}	[{"id": "f0baaece-b045-4d32-aa9b-351d60bbd5e2", "name": "Internet", "subnet_id": "e1263fb7-0f52-4ded-8715-fb2a94f9bf4c", "ip_address": "203.0.113.202", "mac_address": null}]	["8ed08403-eebd-4734-9c3c-d5b8e92c4cc5"]	[{"id": "d74e9df5-f726-43c8-b0a6-8202279f6636", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-28 08:15:22.419904+00	2025-10-28 08:15:22.438328+00
f8fb7ddb-564f-4d02-b120-10ea4167dc32	00145ea8-e012-4015-83ac-00e3ed3e7a0e	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "355fafeb-0afb-4643-945c-6266c1dcf2c1"}	[{"id": "21c33d7e-18e8-4dc0-a707-8614bb3175fa", "name": "Remote Network", "subnet_id": "cb4d9694-70eb-4f03-9d7c-e528c8894c7b", "ip_address": "203.0.113.250", "mac_address": null}]	["7d87d1cd-b7c0-4fdb-b73c-c7c92f066b8c"]	[{"id": "44276c39-c5ae-4d9d-9008-08396e68f06f", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-28 08:15:22.419908+00	2025-10-28 08:15:22.442666+00
868f4a73-4080-49f6-adf3-5436bc6d1e5a	00145ea8-e012-4015-83ac-00e3ed3e7a0e	a6aac04424db	a6aac04424db	NetVisor daemon	{"type": "Hostname"}	[{"id": "d54e429c-76bd-4b58-b195-829fd937361d", "name": "eth0", "subnet_id": "863804d0-6417-4160-bf00-aca65d3ee423", "ip_address": "172.25.0.4", "mac_address": "52:69:41:45:B3:05"}]	["1d52d68b-b50a-41d6-b600-8e13806aa0a1", "29b6fbc1-f86f-442b-8b8e-06cf5e9911c6"]	[{"id": "1c196e55-504f-4485-b257-a1b95932bbc2", "type": "Custom", "number": 60073, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T08:15:40.147808091Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "Network"}, {"date": "2025-10-28T08:15:22.487392055Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "SelfReport"}]}	null	2025-10-28 08:15:22.487393+00	2025-10-28 08:15:58.850949+00
b898051f-631d-4ffb-ab1e-ab2774c605b6	00145ea8-e012-4015-83ac-00e3ed3e7a0e	NetVisor Server API	netvisor-server-1.netvisor_netvisor-dev	\N	{"type": "Hostname"}	[{"id": "7c271c81-283b-45cd-9543-fc359575e248", "name": null, "subnet_id": "863804d0-6417-4160-bf00-aca65d3ee423", "ip_address": "172.25.0.3", "mac_address": "D2:82:77:9C:CB:0D"}]	["a1707348-e417-4b94-b998-d6b9f4ecd57b"]	[{"id": "d5e3cf0b-a034-4e0b-bb8c-c7d1f9b1e5f7", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T08:15:30.503617878Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "Network"}]}	null	2025-10-28 08:15:30.503624+00	2025-10-28 08:15:40.157401+00
ec045cd6-4388-4c4d-9213-e1f00af920ef	00145ea8-e012-4015-83ac-00e3ed3e7a0e	Home Assistant	homeassistant-discovery.netvisor_netvisor-dev	\N	{"type": "Hostname"}	[{"id": "91116886-378c-405b-8843-66df2f12f30c", "name": null, "subnet_id": "863804d0-6417-4160-bf00-aca65d3ee423", "ip_address": "172.25.0.5", "mac_address": "E6:32:BF:18:98:68"}]	["b1e315e9-e8d8-46b2-8ade-e56f1a6e05a6"]	[{"id": "d1d9dd8b-2d21-47e5-8c3a-72979dc5ecee", "type": "Custom", "number": 8123, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T08:15:40.148839508Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "Network"}]}	null	2025-10-28 08:15:40.14884+00	2025-10-28 08:15:58.853514+00
66bd5298-9b3e-4b55-abba-df9b7c1f0e33	00145ea8-e012-4015-83ac-00e3ed3e7a0e	NetVisor Server API	\N	\N	{"type": "None"}	[{"id": "7702d57d-49e7-49b6-b5ef-cb01060541f7", "name": null, "subnet_id": "863804d0-6417-4160-bf00-aca65d3ee423", "ip_address": "172.25.0.1", "mac_address": "E2:48:DF:88:73:13"}]	["d6f26914-a1e5-4d6e-99b5-504181730717", "d6f41ad9-c50d-4421-b72b-8aa8a9896abd"]	[{"id": "e6c620f8-56f1-40e9-8b34-b28f1dd461c6", "type": "Custom", "number": 60072, "protocol": "Tcp"}, {"id": "7c9d4d14-31fc-44a5-a411-b5e305710f66", "type": "Custom", "number": 8123, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T08:15:49.465091387Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "Network"}]}	null	2025-10-28 08:15:49.465092+00	2025-10-28 08:15:58.880715+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
00145ea8-e012-4015-83ac-00e3ed3e7a0e	My Network	2025-10-28 08:15:22.386448+00	2025-10-28 08:15:22.386451+00	t	5da15fcc-a672-48d5-88f3-b79b9ccfe5b7
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, service_definition, virtualization, source) FROM stdin;
28b27aac-26e5-45c1-ba72-de0934250f66	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:22.419902+00	2025-10-28 08:15:22.432745+00	Cloudflare DNS	e70ef2fc-1d4d-45bf-8fb8-c39b00608bd7	[{"id": "a7573bba-fa22-4437-9181-f41104a25793", "type": "Port", "port_id": "69cfac47-1423-4026-ad79-61d02a0fe227", "interface_id": "a112de46-6d90-4fa9-b617-15bd36add619"}]	"Dns Server"	null	{"type": "System"}
8ed08403-eebd-4734-9c3c-d5b8e92c4cc5	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:22.419905+00	2025-10-28 08:15:22.437847+00	Google.com	1a5a585d-df60-469a-8af8-ac3dad035378	[{"id": "e7dfc1e2-7582-4aa3-a859-1a6a4b39269e", "type": "Port", "port_id": "d74e9df5-f726-43c8-b0a6-8202279f6636", "interface_id": "f0baaece-b045-4d32-aa9b-351d60bbd5e2"}]	"Web Service"	null	{"type": "System"}
7d87d1cd-b7c0-4fdb-b73c-c7c92f066b8c	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:22.419909+00	2025-10-28 08:15:22.441833+00	Mobile Device	f8fb7ddb-564f-4d02-b120-10ea4167dc32	[{"id": "355fafeb-0afb-4643-945c-6266c1dcf2c1", "type": "Port", "port_id": "44276c39-c5ae-4d9d-9008-08396e68f06f", "interface_id": "21c33d7e-18e8-4dc0-a707-8614bb3175fa"}]	"Client"	null	{"type": "System"}
1d52d68b-b50a-41d6-b600-8e13806aa0a1	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:22.48742+00	2025-10-28 08:15:58.850016+00	NetVisor Daemon API	868f4a73-4080-49f6-adf3-5436bc6d1e5a	[{"id": "2b4ad6b8-4701-4059-b885-1ca45f80f6aa", "type": "Port", "port_id": "1c196e55-504f-4485-b257-a1b95932bbc2", "interface_id": "d54e429c-76bd-4b58-b195-829fd937361d"}]	"NetVisor Daemon API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Updated match data on 2025-10-28 08:15:40.148188216 UTC", [{"data": "Response from http://172.25.0.4:60073/api/health contained \\"netvisor\\"", "type": "reason"}, {"data": "NetVisor Daemon self-report", "type": "reason"}]], "type": "container"}, "confidence": "Certain"}, "metadata": [{"date": "2025-10-28T08:15:40.148188216Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "Network"}, {"date": "2025-10-28T08:15:22.487412805Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "SelfReport"}]}
a1707348-e417-4b94-b998-d6b9f4ecd57b	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:36.320017+00	2025-10-28 08:15:40.156873+00	NetVisor Server API	b898051f-631d-4ffb-ab1e-ab2774c605b6	[{"id": "e6d00c42-e4e9-4961-aa26-cdb13db95432", "type": "Port", "port_id": "d5e3cf0b-a034-4e0b-bb8c-c7d1f9b1e5f7", "interface_id": "7c271c81-283b-45cd-9543-fc359575e248"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.3:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T08:15:36.319981506Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "Network"}]}
d6f26914-a1e5-4d6e-99b5-504181730717	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:54.921925+00	2025-10-28 08:15:58.850506+00	NetVisor Server API	66bd5298-9b3e-4b55-abba-df9b7c1f0e33	[{"id": "ac69fe8f-b7b5-448c-b318-de13ce044b2b", "type": "Port", "port_id": "e6c620f8-56f1-40e9-8b34-b28f1dd461c6", "interface_id": "7702d57d-49e7-49b6-b5ef-cb01060541f7"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T08:15:54.921903125Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "Network"}]}
b1e315e9-e8d8-46b2-8ade-e56f1a6e05a6	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:47.076275+00	2025-10-28 08:15:58.853005+00	Home Assistant	ec045cd6-4388-4c4d-9213-e1f00af920ef	[{"id": "dd7a3ad7-85b7-400b-ac9f-0f0c2da2618b", "type": "Port", "port_id": "d1d9dd8b-2d21-47e5-8c3a-72979dc5ecee", "interface_id": "91116886-378c-405b-8843-66df2f12f30c"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.5:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T08:15:47.076267761Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "Network"}]}
d6f41ad9-c50d-4421-b72b-8aa8a9896abd	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:56.284522+00	2025-10-28 08:15:58.880137+00	Home Assistant	66bd5298-9b3e-4b55-abba-df9b7c1f0e33	[{"id": "e2394390-43a4-45ce-9217-c06931ab16d7", "type": "Port", "port_id": "7c9d4d14-31fc-44a5-a411-b5e305710f66", "interface_id": "7702d57d-49e7-49b6-b5ef-cb01060541f7"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T08:15:56.284514126Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "Network"}]}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
e1263fb7-0f52-4ded-8715-fb2a94f9bf4c	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:22.419853+00	2025-10-28 08:15:22.419853+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
cb4d9694-70eb-4f03-9d7c-e528c8894c7b	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:22.41986+00	2025-10-28 08:15:22.41986+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
863804d0-6417-4160-bf00-aca65d3ee423	00145ea8-e012-4015-83ac-00e3ed3e7a0e	2025-10-28 08:15:22.476818+00	2025-10-28 08:15:22.476818+00	"172.25.0.0/28"	172.25.0.0/28	\N	"Lan"	{"type": "Discovery", "metadata": [{"date": "2025-10-28T08:15:22.476787972Z", "daemon_id": "d001b4db-368a-485d-81f7-045a854fe593", "discovery_type": "SelfReport"}]}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
5da15fcc-a672-48d5-88f3-b79b9ccfe5b7	Default Username	2025-10-28 08:15:22.385405+00	2025-10-28 08:15:22.385411+00
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

\unrestrict L5177E3frdF5KhlGBkNb9dwbaMUJa1b8cMs1f5hiI854xf9gdvhWcwxtaC82vdg

