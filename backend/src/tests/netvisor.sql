--
-- PostgreSQL database dump
--

\restrict wpHuT1esBw1xGLnTWvEm38XqWPtWc56gavGtkmMoXYGUX9PfyST2GuVop6KKhRC

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
20251006215000	users	2025-10-24 22:17:11.60264+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	1832292
20251006215100	networks	2025-10-24 22:17:11.605304+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	2468750
20251006215151	create hosts	2025-10-24 22:17:11.608002+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	1731833
20251006215155	create subnets	2025-10-24 22:17:11.610025+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	1959125
20251006215201	create groups	2025-10-24 22:17:11.612208+00	t	\\x28b2ad8fa9dd6a96e8798db9c9c5884c91805f5265111ff9dcd3be0222fcb4370a9ae5829600c0281c81540a0c351d90	1639625
20251006215204	create daemons	2025-10-24 22:17:11.614123+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	3378250
20251006215212	create services	2025-10-24 22:17:11.617747+00	t	\\xd96da6bc018518b42c9bcab782539ea9b8c0324448b2889ef3438282ac019131b6493a54f048957613d779da672cd4a8	2357666
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
34ff241d-6898-4688-995b-7e28bd8b3cf1	ba3f8d5a-df84-4608-b308-20190a023525	40f75f03-bba5-4e6d-ae7b-aae304a53b5d	"172.25.0.4"	60073	2025-10-24 22:18:33.460835+00	2025-10-24 22:19:03.465418+00
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
e4adb753-e5a3-483d-a9bf-2c579adcc0d3	ba3f8d5a-df84-4608-b308-20190a023525	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "7d50e2ae-3581-48f7-bc79-3b698dc36f58"}	[{"id": "f96a2bb6-44d9-47fc-b321-b3986fe4187f", "name": "Internet", "subnet_id": "854df085-1c14-409a-97be-9d89df047489", "ip_address": "1.1.1.1", "mac_address": null}]	["4861b926-d346-47b3-91ea-1ce9885582f7"]	[{"id": "92734c9f-5559-4ce6-aced-d0a7268e2617", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-24 22:17:11.624501+00	2025-10-24 22:17:11.664266+00
32f968d2-3c30-4cb8-897b-c449abe6c220	ba3f8d5a-df84-4608-b308-20190a023525	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "0f4db80c-e4d0-4633-a542-a9ccbe354877"}	[{"id": "b22597c5-ea0d-44c1-80c8-65919012c29e", "name": "Internet", "subnet_id": "854df085-1c14-409a-97be-9d89df047489", "ip_address": "203.0.113.104", "mac_address": null}]	["6c30097d-8826-423f-bb52-9e3727d85653"]	[{"id": "f5d7f405-ebd2-45bf-83d5-657627d438b1", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-24 22:17:11.624532+00	2025-10-24 22:17:11.667094+00
f0d79ded-e49a-421c-bd58-142818e14b13	ba3f8d5a-df84-4608-b308-20190a023525	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "489c5354-e5c5-4221-8c7c-3b44b6fe0e79"}	[{"id": "a2922458-2c64-4d97-93bf-5f4284190ffc", "name": "Remote Network", "subnet_id": "b2d43bd4-9124-4d03-b422-48b44a1717df", "ip_address": "203.0.113.72", "mac_address": null}]	["3fbf4416-598d-4009-9569-475efef88564"]	[{"id": "88dd5ea5-6c79-4780-b69a-fe0199a0da7f", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-24 22:17:11.624535+00	2025-10-24 22:17:11.669915+00
9afef49b-83d9-4761-a225-e188476aa05c	ba3f8d5a-df84-4608-b308-20190a023525	NetVisor Server API	netvisor-server-1.netvisor_netvisor-dev	Discovered host	{"type": "Hostname"}	[{"id": "2cc7ca4b-0c11-466d-b9ab-a26c49febf15", "name": null, "subnet_id": "2fdfd51d-9d09-4ee5-99c5-f9335717c6b4", "ip_address": "172.25.0.3", "mac_address": "02:03:F2:85:0E:FC"}]	["a42c43bd-6784-4444-9dbc-8c54a83832d4"]	[{"id": "fde4c1c2-3227-4a57-9969-1d45ffb380a5", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-24T22:18:44.401789178Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}]}	null	2025-10-24 22:18:44.401791+00	2025-10-24 22:19:12.593631+00
40f75f03-bba5-4e6d-ae7b-aae304a53b5d	ba3f8d5a-df84-4608-b308-20190a023525	437d2f09d941	437d2f09d941	NetVisor daemon	{"type": "Hostname"}	[{"id": "addca42d-c6d3-4b42-9e24-93736a071ace", "name": "eth0", "subnet_id": "2fdfd51d-9d09-4ee5-99c5-f9335717c6b4", "ip_address": "172.25.0.4", "mac_address": "DE:D3:66:81:60:CB"}]	["91e0d04d-6627-49b7-a3bd-91e432dfcec4", "2fb84841-3833-4f1f-9d52-bf148d9d6ae0"]	[{"id": "bb6da3d8-7a68-45de-ab3c-fe0ab6fe3dbf", "type": "Custom", "number": 60073, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-24T22:18:44.400201720Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}, {"date": "2025-10-24T22:18:33.455546673Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "SelfReport"}]}	null	2025-10-24 22:18:33.455547+00	2025-10-24 22:18:54.187616+00
4e04e1ad-1e01-45f3-9fd5-d21acc394232	ba3f8d5a-df84-4608-b308-20190a023525	Home Assistant	homeassistant-discovery.netvisor_netvisor-dev	Discovered host	{"type": "Hostname"}	[{"id": "903f90cc-2177-4de8-bbf4-fbe28d2376ae", "name": null, "subnet_id": "2fdfd51d-9d09-4ee5-99c5-f9335717c6b4", "ip_address": "172.25.0.5", "mac_address": "86:70:88:D4:89:4F"}]	["ff265fdd-0813-4767-8d6a-75d0a2491b77"]	[{"id": "2a9ba6b6-b34a-4516-acc6-99d052a28feb", "type": "Custom", "number": 8123, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-24T22:18:54.180789919Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}]}	null	2025-10-24 22:18:54.180795+00	2025-10-24 22:19:12.646098+00
fbd2d3e8-e9f5-4405-9cbb-56da62eeda51	ba3f8d5a-df84-4608-b308-20190a023525	Home Assistant	\N	Discovered host	{"type": "None"}	[{"id": "c32c0fd2-efa1-49fd-b20b-11896973efd4", "name": null, "subnet_id": "2fdfd51d-9d09-4ee5-99c5-f9335717c6b4", "ip_address": "172.25.0.1", "mac_address": "16:CB:8C:6D:FD:57"}]	["a2ae0fe8-9ba6-47c2-bfa7-acdefda8f63a", "64bcf9f5-fc4a-43ad-8234-9e5c688bb6ed", "965224da-d8f0-4802-9ac4-fc7fbf871a69"]	[{"id": "10454d21-1e5f-4e7c-a018-c7135721edf9", "type": "Custom", "number": 8123, "protocol": "Tcp"}, {"id": "00d1c682-480f-4ab9-930e-07f83b8aad56", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-24T22:19:03.627769548Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}]}	null	2025-10-24 22:19:03.62777+00	2025-10-24 22:19:12.6488+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
ba3f8d5a-df84-4608-b308-20190a023525	My Network	2025-10-24 22:17:11.62278+00	2025-10-24 22:17:11.622782+00	t	85ad839a-0f98-4d87-b794-a7bba0af1d5d
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, is_gateway, service_definition, virtualization, source) FROM stdin;
4861b926-d346-47b3-91ea-1ce9885582f7	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:17:11.624528+00	2025-10-24 22:17:11.663723+00	Cloudflare DNS	e4adb753-e5a3-483d-a9bf-2c579adcc0d3	[{"id": "7d50e2ae-3581-48f7-bc79-3b698dc36f58", "type": "Port", "port_id": "92734c9f-5559-4ce6-aced-d0a7268e2617", "interface_id": "f96a2bb6-44d9-47fc-b321-b3986fe4187f"}]	f	"Dns Server"	null	{"type": "System"}
6c30097d-8826-423f-bb52-9e3727d85653	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:17:11.624533+00	2025-10-24 22:17:11.666713+00	Google.com	32f968d2-3c30-4cb8-897b-c449abe6c220	[{"id": "0f4db80c-e4d0-4633-a542-a9ccbe354877", "type": "Port", "port_id": "f5d7f405-ebd2-45bf-83d5-657627d438b1", "interface_id": "b22597c5-ea0d-44c1-80c8-65919012c29e"}]	f	"Web Service"	null	{"type": "System"}
3fbf4416-598d-4009-9569-475efef88564	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:17:11.624535+00	2025-10-24 22:17:11.669523+00	Mobile Device	f0d79ded-e49a-421c-bd58-142818e14b13	[{"id": "489c5354-e5c5-4221-8c7c-3b44b6fe0e79", "type": "Port", "port_id": "88dd5ea5-6c79-4780-b69a-fe0199a0da7f", "interface_id": "a2922458-2c64-4d97-93bf-5f4284190ffc"}]	f	"Client"	null	{"type": "System"}
ff265fdd-0813-4767-8d6a-75d0a2491b77	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:18:57.334927+00	2025-10-24 22:19:12.645384+00	Home Assistant	4e04e1ad-1e01-45f3-9fd5-d21acc394232	[{"id": "b6e8891c-95df-43f0-b739-7c74ef48e1e2", "type": "Port", "port_id": "2a9ba6b6-b34a-4516-acc6-99d052a28feb", "interface_id": "903f90cc-2177-4de8-bbf4-fbe28d2376ae"}]	f	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.5:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-24T22:18:57.334900378Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}]}
a2ae0fe8-9ba6-47c2-bfa7-acdefda8f63a	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:19:06.32601+00	2025-10-24 22:19:12.647986+00	Home Assistant	fbd2d3e8-e9f5-4405-9cbb-56da62eeda51	[{"id": "2318c320-ef6e-4083-93ff-1ac599b59dd0", "type": "Port", "port_id": "10454d21-1e5f-4e7c-a018-c7135721edf9", "interface_id": "c32c0fd2-efa1-49fd-b20b-11896973efd4"}]	t	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-24T22:19:06.325995591Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}]}
91e0d04d-6627-49b7-a3bd-91e432dfcec4	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:18:33.455552+00	2025-10-24 22:18:54.186991+00	NetVisor Daemon API	40f75f03-bba5-4e6d-ae7b-aae304a53b5d	[{"id": "8da8ffd9-58ac-45f2-8048-d2652c55de5f", "type": "Port", "port_id": "bb6da3d8-7a68-45de-ab3c-fe0ab6fe3dbf", "interface_id": "addca42d-c6d3-4b42-9e24-93736a071ace"}]	f	"NetVisor Daemon API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Updated match data on 2025-10-24 22:18:44.400985803 UTC", [{"data": "Response from http://172.25.0.4:60073/api/health contained \\"netvisor\\"", "type": "reason"}, {"data": "NetVisor Daemon self-report", "type": "reason"}]], "type": "container"}, "confidence": "Certain"}, "metadata": [{"date": "2025-10-24T22:18:44.400985803Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}, {"date": "2025-10-24T22:18:33.455552173Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "SelfReport"}]}
a42c43bd-6784-4444-9dbc-8c54a83832d4	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:18:47.83451+00	2025-10-24 22:19:12.593272+00	NetVisor Server API	9afef49b-83d9-4761-a225-e188476aa05c	[{"id": "5d9b7cd8-3f39-4e00-8508-d0f6dbb709b2", "type": "Port", "port_id": "fde4c1c2-3227-4a57-9969-1d45ffb380a5", "interface_id": "2cc7ca4b-0c11-466d-b9ab-a26c49febf15"}]	f	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.3:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-24T22:18:47.834483138Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}]}
64bcf9f5-fc4a-43ad-8234-9e5c688bb6ed	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:19:06.766029+00	2025-10-24 22:19:12.648059+00	NetVisor Server API	fbd2d3e8-e9f5-4405-9cbb-56da62eeda51	[{"id": "1c4949d9-10e6-48d5-b0e2-6e6b124076ee", "type": "Port", "port_id": "00d1c682-480f-4ab9-930e-07f83b8aad56", "interface_id": "c32c0fd2-efa1-49fd-b20b-11896973efd4"}]	t	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-24T22:19:06.766016175Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}]}
965224da-d8f0-4802-9ac4-fc7fbf871a69	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:19:12.558433+00	2025-10-24 22:19:12.648344+00	Gateway	fbd2d3e8-e9f5-4405-9cbb-56da62eeda51	[{"id": "125126af-8850-4bfe-9290-13cca576bd7a", "type": "Interface", "interface_id": "c32c0fd2-efa1-49fd-b20b-11896973efd4"}]	t	"Gateway"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Match confidence for generic services is N/A", [{"data": ["All of", [{"data": "Host IP address is in routing table of daemon 34ff241d-6898-4688-995b-7e28bd8b3cf1", "type": "reason"}, {"data": "No other gateway services matched", "type": "reason"}]], "type": "container"}]], "type": "container"}, "confidence": "NotApplicable"}, "metadata": [{"date": "2025-10-24T22:19:12.558422636Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "Network"}]}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
854df085-1c14-409a-97be-9d89df047489	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:17:11.624427+00	2025-10-24 22:17:11.624427+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
b2d43bd4-9124-4d03-b422-48b44a1717df	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:17:11.624432+00	2025-10-24 22:17:11.624432+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
2fdfd51d-9d09-4ee5-99c5-f9335717c6b4	ba3f8d5a-df84-4608-b308-20190a023525	2025-10-24 22:18:33.352791+00	2025-10-24 22:18:33.352791+00	"172.25.0.0/28"	172.25.0.0/28	\N	"Lan"	{"type": "Discovery", "metadata": [{"date": "2025-10-24T22:18:33.352781006Z", "daemon_id": "34ff241d-6898-4688-995b-7e28bd8b3cf1", "discovery_type": "SelfReport"}]}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
85ad839a-0f98-4d87-b794-a7bba0af1d5d	Default Username	2025-10-24 22:17:11.622036+00	2025-10-24 22:17:11.622043+00
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

\unrestrict wpHuT1esBw1xGLnTWvEm38XqWPtWc56gavGtkmMoXYGUX9PfyST2GuVop6KKhRC

