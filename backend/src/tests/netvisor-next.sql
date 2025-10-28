--
-- PostgreSQL database dump
--

\restrict MoIiJaewocNyaE0o50mcwJs4rbc9rpOW3Q6UBzUxObkJyGgGfBoDca0Yu2cJa7Y

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
20251006215000	users	2025-10-28 16:16:00.367123+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	1868333
20251006215100	networks	2025-10-28 16:16:00.369757+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	2099167
20251006215151	create hosts	2025-10-28 16:16:00.372058+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	1412167
20251006215155	create subnets	2025-10-28 16:16:00.373635+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	1869583
20251006215201	create groups	2025-10-28 16:16:00.375695+00	t	\\x0a7032bf4d33a0baf020e905da865cde240e2a09dda2f62aa535b2c5d4b26b20be30a3286f1b5192bd94cd4a5dbb5bcd	1435625
20251006215204	create daemons	2025-10-28 16:16:00.377322+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	1624166
20251006215212	create services	2025-10-28 16:16:00.379145+00	t	\\xd5b07f82fc7c9da2782a364d46078d7d16b5c08df70cfbf02edcfe9b1b24ab6024ad159292aeea455f15cfd1f4740c1d	1747583
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
cc8bd168-2dfe-43ce-899d-1dbc30e19090	6c94205f-6020-4c34-a69e-3293e4222098	8b6aab8e-7872-4af3-b941-a6c063f14ba7	"172.25.0.4"	60073	2025-10-28 16:16:00.462162+00	2025-10-28 16:16:37.057767+00
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
5799c975-a994-4b04-8350-3ed033ab0e30	6c94205f-6020-4c34-a69e-3293e4222098	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "e48737a3-c5fd-4c04-b848-38f7c4d91ef7"}	[{"id": "b3666e10-c86f-4134-b8c2-3991d2ef4e4c", "name": "Internet", "subnet_id": "24d6b7c6-0c10-4f29-abed-6312f340fd44", "ip_address": "1.1.1.1", "mac_address": null}]	["0c39026b-6138-486e-8d31-1164b8ac131e"]	[{"id": "d921f953-55c8-4112-a878-5376b6658c19", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-28 16:16:00.415959+00	2025-10-28 16:16:00.424334+00
1b66b84f-6cbc-40b8-ad7e-cf90d2689efa	6c94205f-6020-4c34-a69e-3293e4222098	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "bc04824d-8400-4f4d-a41f-03b78804a50d"}	[{"id": "52d9c195-1c52-4637-ab59-7b6610ea79f1", "name": "Internet", "subnet_id": "24d6b7c6-0c10-4f29-abed-6312f340fd44", "ip_address": "203.0.113.103", "mac_address": null}]	["b7f37bcc-6418-4f03-b219-836cd30a1ab7"]	[{"id": "7cee49a7-54e6-48a8-b951-7fb3fdacf1f4", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-28 16:16:00.415963+00	2025-10-28 16:16:00.427165+00
af0b4c95-ed55-4208-b1c5-02ba21c78eec	6c94205f-6020-4c34-a69e-3293e4222098	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "b78ff125-b28c-42cc-ba06-979eb33a87ac"}	[{"id": "8b28c89b-5a46-4043-a280-899b4c14f8f9", "name": "Remote Network", "subnet_id": "a947dc17-7b33-4cf1-9c6a-bab9cda65c13", "ip_address": "203.0.113.65", "mac_address": null}]	["c2605ac5-586f-4bd8-8806-55688edf695b"]	[{"id": "aec59fe3-49f9-47e8-8905-175e3962c014", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-28 16:16:00.415965+00	2025-10-28 16:16:00.429844+00
218aa7a2-8d66-41df-8498-f7852677be3a	6c94205f-6020-4c34-a69e-3293e4222098	NetVisor Server API	netvisor-server-1.netvisor_netvisor-dev	\N	{"type": "Hostname"}	[{"id": "88201dd5-3eec-43a1-a58f-7ec5d24fc1fd", "name": null, "subnet_id": "f9e9b04e-77a8-4623-a118-b25195e3e0e3", "ip_address": "172.25.0.3", "mac_address": "26:41:50:4B:B3:91"}]	["f127712b-3298-4a43-bb7a-efcbb865bcfd"]	[{"id": "b64414e2-566c-46e4-b1a1-f291cc596611", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T16:16:09.807560959Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "Network"}]}	null	2025-10-28 16:16:09.807563+00	2025-10-28 16:16:19.289753+00
8b6aab8e-7872-4af3-b941-a6c063f14ba7	6c94205f-6020-4c34-a69e-3293e4222098	6341a67c8ea6	6341a67c8ea6	NetVisor daemon	{"type": "Hostname"}	[{"id": "052537ed-8468-416b-b180-b76fb3694bd5", "name": "eth0", "subnet_id": "f9e9b04e-77a8-4623-a118-b25195e3e0e3", "ip_address": "172.25.0.4", "mac_address": "96:79:46:2A:38:39"}]	["28dc8d74-44cf-4f0d-a7fb-84b4f5a9dce8", "e79fd895-147e-44c8-a651-86157950a67f"]	[{"id": "0b4d0567-9c7f-4663-a4d2-aa186188360d", "type": "Custom", "number": 60073, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T16:16:09.803392876Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "Network"}, {"date": "2025-10-28T16:16:00.454137844Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "SelfReport"}]}	null	2025-10-28 16:16:00.454139+00	2025-10-28 16:16:09.818928+00
4396f1c5-9f55-413c-965b-c1d824cf3867	6c94205f-6020-4c34-a69e-3293e4222098	Home Assistant	homeassistant-discovery.netvisor_netvisor-dev	\N	{"type": "Hostname"}	[{"id": "60f7d481-473e-42d4-891f-6968739581a4", "name": null, "subnet_id": "f9e9b04e-77a8-4623-a118-b25195e3e0e3", "ip_address": "172.25.0.5", "mac_address": "96:A1:0A:28:68:12"}]	["a188e8df-66ac-4a97-be05-36c39bbc1d9f"]	[{"id": "c5080ea5-14da-44c9-82a6-669a635c5f83", "type": "Custom", "number": 8123, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T16:16:19.277217338Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "Network"}]}	null	2025-10-28 16:16:19.277218+00	2025-10-28 16:16:37.818013+00
e73f2772-bd40-4f20-bffd-a674ee7a7fe2	6c94205f-6020-4c34-a69e-3293e4222098	Home Assistant	\N	\N	{"type": "None"}	[{"id": "2664b36c-7e1d-4f81-8dba-0933171abeee", "name": null, "subnet_id": "f9e9b04e-77a8-4623-a118-b25195e3e0e3", "ip_address": "172.25.0.1", "mac_address": "7E:FA:AC:DB:D7:B1"}]	["bc0fcc37-33ab-4ac5-9fdf-e0aaca02ef2e", "d3d93128-e43f-4f98-8483-53018a9f7fdc"]	[{"id": "19c18709-e37c-4238-947b-19dd78b34714", "type": "Custom", "number": 8123, "protocol": "Tcp"}, {"id": "983cc4fe-3643-4ed4-9204-ea79aef4fab1", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T16:16:28.267119343Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "Network"}]}	null	2025-10-28 16:16:28.267121+00	2025-10-28 16:16:37.82282+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
6c94205f-6020-4c34-a69e-3293e4222098	My Network	2025-10-28 16:16:00.383674+00	2025-10-28 16:16:00.383676+00	t	77242d85-7154-4a95-aeb7-2db4bad6ede7
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, service_definition, virtualization, source) FROM stdin;
0c39026b-6138-486e-8d31-1164b8ac131e	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:00.41596+00	2025-10-28 16:16:00.423813+00	Cloudflare DNS	5799c975-a994-4b04-8350-3ed033ab0e30	[{"id": "e48737a3-c5fd-4c04-b848-38f7c4d91ef7", "type": "Port", "port_id": "d921f953-55c8-4112-a878-5376b6658c19", "interface_id": "b3666e10-c86f-4134-b8c2-3991d2ef4e4c"}]	"Dns Server"	null	{"type": "System"}
b7f37bcc-6418-4f03-b219-836cd30a1ab7	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:00.415964+00	2025-10-28 16:16:00.426819+00	Google.com	1b66b84f-6cbc-40b8-ad7e-cf90d2689efa	[{"id": "bc04824d-8400-4f4d-a41f-03b78804a50d", "type": "Port", "port_id": "7cee49a7-54e6-48a8-b951-7fb3fdacf1f4", "interface_id": "52d9c195-1c52-4637-ab59-7b6610ea79f1"}]	"Web Service"	null	{"type": "System"}
c2605ac5-586f-4bd8-8806-55688edf695b	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:00.415966+00	2025-10-28 16:16:00.429527+00	Mobile Device	af0b4c95-ed55-4208-b1c5-02ba21c78eec	[{"id": "b78ff125-b28c-42cc-ba06-979eb33a87ac", "type": "Port", "port_id": "aec59fe3-49f9-47e8-8905-175e3962c014", "interface_id": "8b28c89b-5a46-4043-a280-899b4c14f8f9"}]	"Client"	null	{"type": "System"}
a188e8df-66ac-4a97-be05-36c39bbc1d9f	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:25.087326+00	2025-10-28 16:16:37.791051+00	Home Assistant	4396f1c5-9f55-413c-965b-c1d824cf3867	[{"id": "3b1f0ccc-1df9-4840-a20f-b6214f71694b", "type": "Port", "port_id": "c5080ea5-14da-44c9-82a6-669a635c5f83", "interface_id": "60f7d481-473e-42d4-891f-6968739581a4"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.5:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T16:16:25.087320758Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "Network"}]}
28dc8d74-44cf-4f0d-a7fb-84b4f5a9dce8	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:00.454161+00	2025-10-28 16:16:09.818057+00	NetVisor Daemon API	8b6aab8e-7872-4af3-b941-a6c063f14ba7	[{"id": "eaf40be1-4951-49fd-89db-76e9d3cb6199", "type": "Port", "port_id": "0b4d0567-9c7f-4663-a4d2-aa186188360d", "interface_id": "052537ed-8468-416b-b180-b76fb3694bd5"}]	"NetVisor Daemon API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Updated match data on 2025-10-28 16:16:09.803936292 UTC", [{"data": "Response from http://172.25.0.4:60073/api/health contained \\"netvisor\\"", "type": "reason"}, {"data": "NetVisor Daemon self-report", "type": "reason"}]], "type": "container"}, "confidence": "Certain"}, "metadata": [{"date": "2025-10-28T16:16:09.803936292Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "Network"}, {"date": "2025-10-28T16:16:00.454157969Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "SelfReport"}]}
f127712b-3298-4a43-bb7a-efcbb865bcfd	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:17.919452+00	2025-10-28 16:16:19.289272+00	NetVisor Server API	218aa7a2-8d66-41df-8498-f7852677be3a	[{"id": "281fb9fa-354f-4f9e-9c60-c95bd6b0a8db", "type": "Port", "port_id": "b64414e2-566c-46e4-b1a1-f291cc596611", "interface_id": "88201dd5-3eec-43a1-a58f-7ec5d24fc1fd"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.3:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T16:16:17.919409004Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "Network"}]}
bc0fcc37-33ab-4ac5-9fdf-e0aaca02ef2e	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:34.049057+00	2025-10-28 16:16:37.82204+00	Home Assistant	e73f2772-bd40-4f20-bffd-a674ee7a7fe2	[{"id": "d89af791-2796-4518-a141-7f3a7bfe876d", "type": "Port", "port_id": "19c18709-e37c-4238-947b-19dd78b34714", "interface_id": "2664b36c-7e1d-4f81-8dba-0933171abeee"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T16:16:34.049050637Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "Network"}]}
d3d93128-e43f-4f98-8483-53018a9f7fdc	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:36.193367+00	2025-10-28 16:16:37.822205+00	NetVisor Server API	e73f2772-bd40-4f20-bffd-a674ee7a7fe2	[{"id": "57af4246-7039-40e4-b8d4-df5fbad81a42", "type": "Port", "port_id": "983cc4fe-3643-4ed4-9204-ea79aef4fab1", "interface_id": "2664b36c-7e1d-4f81-8dba-0933171abeee"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T16:16:36.193351221Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "Network"}]}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
24d6b7c6-0c10-4f29-abed-6312f340fd44	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:00.415919+00	2025-10-28 16:16:00.415919+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
a947dc17-7b33-4cf1-9c6a-bab9cda65c13	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:00.415925+00	2025-10-28 16:16:00.415925+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
f9e9b04e-77a8-4623-a118-b25195e3e0e3	6c94205f-6020-4c34-a69e-3293e4222098	2025-10-28 16:16:00.444753+00	2025-10-28 16:16:00.444753+00	"172.25.0.0/28"	172.25.0.0/28	\N	"Lan"	{"type": "Discovery", "metadata": [{"date": "2025-10-28T16:16:00.444741677Z", "daemon_id": "cc8bd168-2dfe-43ce-899d-1dbc30e19090", "discovery_type": "SelfReport"}]}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
77242d85-7154-4a95-aeb7-2db4bad6ede7	Default Username	2025-10-28 16:16:00.383189+00	2025-10-28 16:16:00.383191+00
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

\unrestrict MoIiJaewocNyaE0o50mcwJs4rbc9rpOW3Q6UBzUxObkJyGgGfBoDca0Yu2cJa7Y

