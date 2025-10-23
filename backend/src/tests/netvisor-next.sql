--
-- PostgreSQL database dump
--

\restrict KnM3D7E2HZeYSHoxfLQzhGeK0mFlbRxBkQORKUIwtf3yg4azLRub0HGB5cnsgiG

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
    vms jsonb,
    containers jsonb,
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
20251006215000	users	2025-10-23 14:58:46.031154+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	3178167
20251006215100	networks	2025-10-23 14:58:46.035784+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	5161958
20251006215151	create hosts	2025-10-23 14:58:46.041616+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	8275625
20251006215155	create subnets	2025-10-23 14:58:46.050484+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	2453667
20251006215201	create groups	2025-10-23 14:58:46.053643+00	t	\\x96cdc35b7ad03869a836d4a4fe8c3060d075c32edce248827903ceab5c4e41b0727300d6c5755e54973f3ada9e50293a	4967125
20251006215204	create daemons	2025-10-23 14:58:46.059179+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	4182000
20251006215212	create services	2025-10-23 14:58:46.063674+00	t	\\xe92885a5c8ea6bfa00c702c1aa81960a54704ab7219223ff469c3e6f2517ffe75dbed8bb4efd87de79fb3cf2c86e5c23	2742750
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
1234174f-3412-4553-832b-c5d78c1e9044	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	83dd2ca2-e4f7-48be-ac8e-754eb3199223	"172.25.0.4"	60073	2025-10-23 15:00:28.407836+00	2025-10-23 15:00:58.419035+00
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
8c05dd5c-b2db-42b2-86c9-01fbc7ad1652	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "10267f7b-3ad7-4a35-a450-f3e8182f5398"}	[{"id": "b30c170a-af46-4e43-8467-cfc31c30c062", "name": "Internet", "subnet_id": "4f7444aa-a00e-457c-9b72-d9611441442a", "ip_address": "1.1.1.1", "mac_address": null}]	["b2659052-b2eb-41ce-a7fc-2be680cb7e2b"]	[{"id": "6b4306a0-d1c6-48bb-8a5e-b069a6ba9250", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-23 14:58:46.08081+00	2025-10-23 14:58:46.089607+00
68499142-b183-4617-b25a-8af81db2d3fb	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "a0d557f5-ee60-4d2a-a18f-58162c4761b5"}	[{"id": "7bda4344-f3af-42ad-8bf8-7c77c29f17c8", "name": "Internet", "subnet_id": "4f7444aa-a00e-457c-9b72-d9611441442a", "ip_address": "203.0.113.247", "mac_address": null}]	["b79ab043-a62f-40f2-821a-c5993a8d743c"]	[{"id": "b0ed801f-37b8-4729-a987-fee0a6f6bf48", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-23 14:58:46.080812+00	2025-10-23 14:58:46.093865+00
99ce722d-3e2d-422a-9f6f-d078eec4c8ca	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "f9434cd7-81dd-4e9a-9ede-ce9f234bad18"}	[{"id": "a4777e9b-876c-44d0-827a-38540d79560b", "name": "Remote Network", "subnet_id": "5af15c2d-1674-4225-9d94-4845ab2c2d21", "ip_address": "203.0.113.139", "mac_address": null}]	["b12fcfcf-2ef1-4fcb-8027-def7b77773cd"]	[{"id": "f7f094af-34da-4886-86e7-121c58499ac4", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-23 14:58:46.080814+00	2025-10-23 14:58:46.096906+00
83dd2ca2-e4f7-48be-ac8e-754eb3199223	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	8b2edc592b5b	8b2edc592b5b	NetVisor daemon	{"type": "Hostname"}	[{"id": "475c0e6a-9eca-4d0b-93a3-1fb70c7b6c70", "name": "eth0", "subnet_id": "d7244b9c-d2c5-4e35-a101-7b5df75d9fff", "ip_address": "172.25.0.4", "mac_address": "6A:3E:10:F9:DD:65"}]	["81e12ba0-cda7-42ed-ba91-00a6a4462988", "f801dad5-1ac7-4c99-8e45-e89a610b5532"]	[{"id": "669e20d7-fa1c-4114-8324-f0974c72100a", "type": "Custom", "number": 60073, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-23T15:00:44.929750213Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}, {"date": "2025-10-23T15:00:28.395699719Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "SelfReport"}]}	null	2025-10-23 15:00:28.395701+00	2025-10-23 15:01:03.092122+00
80986798-88a4-4a46-971c-fc3f4d3d0736	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	Home Assistant	\N	Discovered host	{"type": "None"}	[{"id": "25fedb8a-f802-463a-aa29-b73e017df6c2", "name": null, "subnet_id": "d7244b9c-d2c5-4e35-a101-7b5df75d9fff", "ip_address": "172.25.0.1", "mac_address": "8E:56:F1:95:E2:BB"}]	["36297230-b2a6-4367-8581-e26c79782be6", "46253287-c854-4cd9-b19c-4f7781d7b0e2", "7fcc91e5-7bc0-4a9a-8d45-9add37d2dc66"]	[{"id": "d3f93254-73ea-4dd0-9dcc-3e5de5755e22", "type": "Custom", "number": 8123, "protocol": "Tcp"}, {"id": "88c49afe-8aa5-403d-95f7-477d4e3bbca4", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-23T15:00:53.957721675Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}]}	null	2025-10-23 15:00:53.957724+00	2025-10-23 15:01:03.104964+00
685c8727-f11d-480e-9db0-fe481de84191	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	NetVisor Server API	netvisor-server-1.netvisor_netvisor-dev	Discovered host	{"type": "Hostname"}	[{"id": "ed8b27ea-9fbb-48f5-8017-367ba9da8a9c", "name": null, "subnet_id": "d7244b9c-d2c5-4e35-a101-7b5df75d9fff", "ip_address": "172.25.0.3", "mac_address": "3A:13:30:B4:D4:06"}]	["f58d108b-18e4-4fdf-a585-428bf7af8998"]	[{"id": "344e262d-9f73-4cbc-b5cb-c50e58fc45aa", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-23T15:00:35.477809208Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}]}	null	2025-10-23 15:00:35.477818+00	2025-10-23 15:01:03.092395+00
ac65b692-9ce3-416c-8a1c-4678010f386c	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	Home Assistant	homeassistant-discovery.netvisor_netvisor-dev	Discovered host	{"type": "Hostname"}	[{"id": "2bd92e47-76c7-4bd3-a66f-d8de4b8f60f2", "name": null, "subnet_id": "d7244b9c-d2c5-4e35-a101-7b5df75d9fff", "ip_address": "172.25.0.5", "mac_address": "2E:89:B6:D7:4D:3D"}]	["12fbbc15-b4d0-4866-b3e8-2a9a6fa49f22"]	[{"id": "03a435a8-cc69-4c0e-a663-ada030ada3d2", "type": "Custom", "number": 8123, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-23T15:00:44.930473588Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}]}	null	2025-10-23 15:00:44.930474+00	2025-10-23 15:01:03.098896+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
cbfa57fe-5bc0-4715-bd2e-8c19796580fd	My Network	2025-10-23 14:58:46.068637+00	2025-10-23 14:58:46.068638+00	t	42bd1949-b2ea-4b2b-93bc-b34602bbd258
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, service_definition, virtualization, vms, containers, source) FROM stdin;
b2659052-b2eb-41ce-a7fc-2be680cb7e2b	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 14:58:46.08081+00	2025-10-23 14:58:46.08081+00	Cloudflare DNS	8c05dd5c-b2db-42b2-86c9-01fbc7ad1652	[{"id": "10267f7b-3ad7-4a35-a450-f3e8182f5398", "type": "Layer4", "port_id": "6b4306a0-d1c6-48bb-8a5e-b069a6ba9250", "interface_id": "b30c170a-af46-4e43-8467-cfc31c30c062"}]	"Dns Server"	null	[]	[]	{"type": "System"}
b79ab043-a62f-40f2-821a-c5993a8d743c	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 14:58:46.080813+00	2025-10-23 14:58:46.080813+00	Google.com	68499142-b183-4617-b25a-8af81db2d3fb	[{"id": "a0d557f5-ee60-4d2a-a18f-58162c4761b5", "type": "Layer4", "port_id": "b0ed801f-37b8-4729-a987-fee0a6f6bf48", "interface_id": "7bda4344-f3af-42ad-8bf8-7c77c29f17c8"}]	"Web Service"	null	[]	[]	{"type": "System"}
b12fcfcf-2ef1-4fcb-8027-def7b77773cd	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 14:58:46.080814+00	2025-10-23 14:58:46.080814+00	Mobile Device	99ce722d-3e2d-422a-9f6f-d078eec4c8ca	[{"id": "f9434cd7-81dd-4e9a-9ede-ce9f234bad18", "type": "Layer4", "port_id": "f7f094af-34da-4886-86e7-121c58499ac4", "interface_id": "a4777e9b-876c-44d0-827a-38540d79560b"}]	"Client"	null	[]	[]	{"type": "System"}
81e12ba0-cda7-42ed-ba91-00a6a4462988	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 15:00:28.39571+00	2025-10-23 15:00:28.39571+00	NetVisor Daemon API	83dd2ca2-e4f7-48be-ac8e-754eb3199223	[{"id": "cae8c36f-f90d-41a3-b4bd-ade111de5613", "type": "Layer4", "port_id": "669e20d7-fa1c-4114-8324-f0974c72100a", "interface_id": "475c0e6a-9eca-4d0b-93a3-1fb70c7b6c70"}]	"NetVisor Daemon API"	null	[]	[]	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Updated match data on 2025-10-23 15:00:44.929941005 UTC", [{"data": "Response from http://172.25.0.4:60073/api/health contained \\"netvisor\\"", "type": "reason"}, {"data": "NetVisor Daemon self-report", "type": "reason"}]], "type": "container"}, "confidence": "Certain"}, "metadata": [{"date": "2025-10-23T15:00:44.929941005Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}, {"date": "2025-10-23T15:00:28.395707261Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "SelfReport"}]}
f58d108b-18e4-4fdf-a585-428bf7af8998	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 15:00:44.00293+00	2025-10-23 15:00:44.00293+00	NetVisor Server API	685c8727-f11d-480e-9db0-fe481de84191	[{"id": "768ad9cd-97c6-49e7-b059-ae0f6c60df3c", "type": "Layer4", "port_id": "344e262d-9f73-4cbc-b5cb-c50e58fc45aa", "interface_id": "ed8b27ea-9fbb-48f5-8017-367ba9da8a9c"}]	"NetVisor Server API"	null	[]	[]	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.3:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-23T15:00:44.002606462Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}]}
46253287-c854-4cd9-b19c-4f7781d7b0e2	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 15:01:02.184634+00	2025-10-23 15:01:02.184634+00	NetVisor Server API	80986798-88a4-4a46-971c-fc3f4d3d0736	[{"id": "c3bb4b81-eeb6-45e0-82ed-269ee72cb013", "type": "Layer4", "port_id": "88c49afe-8aa5-403d-95f7-477d4e3bbca4", "interface_id": "25fedb8a-f802-463a-aa29-b73e017df6c2"}]	"NetVisor Server API"	null	[]	[]	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-23T15:01:02.184567388Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}]}
7fcc91e5-7bc0-4a9a-8d45-9add37d2dc66	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 15:01:03.080685+00	2025-10-23 15:01:03.080685+00	Gateway	80986798-88a4-4a46-971c-fc3f4d3d0736	[{"id": "89c7120b-7ecc-44ac-bdac-d4a80a6025a0", "type": "Layer3", "interface_id": "25fedb8a-f802-463a-aa29-b73e017df6c2"}]	"Gateway"	null	[]	[]	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Match confidence for generic services is N/A", [{"data": "Host IP address is in routing table of daemon 1234174f-3412-4553-832b-c5d78c1e9044", "type": "reason"}]], "type": "container"}, "confidence": "NotApplicable"}, "metadata": [{"date": "2025-10-23T15:01:03.080567596Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}]}
12fbbc15-b4d0-4866-b3e8-2a9a6fa49f22	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 15:00:47.179442+00	2025-10-23 15:00:47.179442+00	Home Assistant	ac65b692-9ce3-416c-8a1c-4678010f386c	[{"id": "48dfa35e-4655-48df-9173-98daa5a8d96b", "type": "Layer4", "port_id": "03a435a8-cc69-4c0e-a663-ada030ada3d2", "interface_id": "2bd92e47-76c7-4bd3-a66f-d8de4b8f60f2"}]	"Home Assistant"	null	[]	[]	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.5:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-23T15:00:47.179376922Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}]}
36297230-b2a6-4367-8581-e26c79782be6	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 15:00:56.178285+00	2025-10-23 15:00:56.178285+00	Home Assistant	80986798-88a4-4a46-971c-fc3f4d3d0736	[{"id": "8c728997-fa86-4f65-aea9-f4e9de75c2e3", "type": "Layer4", "port_id": "d3f93254-73ea-4dd0-9dcc-3e5de5755e22", "interface_id": "25fedb8a-f802-463a-aa29-b73e017df6c2"}]	"Home Assistant"	null	[]	[]	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-23T15:00:56.178228177Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "Network"}]}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
4f7444aa-a00e-457c-9b72-d9611441442a	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 14:58:46.080801+00	2025-10-23 14:58:46.080801+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
5af15c2d-1674-4225-9d94-4845ab2c2d21	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 14:58:46.080803+00	2025-10-23 14:58:46.080803+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
d7244b9c-d2c5-4e35-a101-7b5df75d9fff	cbfa57fe-5bc0-4715-bd2e-8c19796580fd	2025-10-23 15:00:28.285066+00	2025-10-23 15:00:28.285066+00	"172.25.0.0/28"	172.25.0.0/28	\N	"Lan"	{"type": "Discovery", "metadata": [{"date": "2025-10-23T15:00:28.285035344Z", "daemon_id": "1234174f-3412-4553-832b-c5d78c1e9044", "discovery_type": "SelfReport"}]}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
42bd1949-b2ea-4b2b-93bc-b34602bbd258	Default Username	2025-10-23 14:58:46.067724+00	2025-10-23 14:58:46.067725+00
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

\unrestrict KnM3D7E2HZeYSHoxfLQzhGeK0mFlbRxBkQORKUIwtf3yg4azLRub0HGB5cnsgiG

