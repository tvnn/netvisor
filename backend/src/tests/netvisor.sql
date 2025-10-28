--
-- PostgreSQL database dump
--

\restrict MV3NY8WgL1UFlvKW6bYyBYrNEdo0HTiizKt11yhSEyxghvly8JkJtCFgGilMLUJ

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
20251006215000	users	2025-10-28 01:23:57.8754+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	1906625
20251006215100	networks	2025-10-28 01:23:57.877906+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	2307375
20251006215151	create hosts	2025-10-28 01:23:57.880423+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	1478791
20251006215155	create subnets	2025-10-28 01:23:57.882073+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	2233167
20251006215201	create groups	2025-10-28 01:23:57.88446+00	t	\\x0a7032bf4d33a0baf020e905da865cde240e2a09dda2f62aa535b2c5d4b26b20be30a3286f1b5192bd94cd4a5dbb5bcd	1428792
20251006215204	create daemons	2025-10-28 01:23:57.88605+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	1550167
20251006215212	create services	2025-10-28 01:23:57.88777+00	t	\\xd5b07f82fc7c9da2782a364d46078d7d16b5c08df70cfbf02edcfe9b1b24ab6024ad159292aeea455f15cfd1f4740c1d	3314709
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, network_id, host_id, ip, port, registered_at, last_seen) FROM stdin;
2079500c-2e83-4176-9969-b7de96b2bc9d	fa7742a1-5f18-4e4d-a336-0935a803df22	c7a127e3-5b88-4e39-a5f6-00e2dd790e0d	"172.25.0.4"	60073	2025-10-28 01:26:44.245982+00	2025-10-28 01:27:14.252192+00
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
e1abd7cf-d901-40b1-b66e-9618b9ba7a12	fa7742a1-5f18-4e4d-a336-0935a803df22	Cloudflare DNS	\N	Cloudflare DNS	{"type": "ServiceBinding", "config": "434472b0-6d22-48be-8e12-23e86fc052c6"}	[{"id": "8d9ed935-bbf8-4466-a89b-d574a35eb1f5", "name": "Internet", "subnet_id": "29b0bf4c-957a-410d-9b35-e1b2c620e3ec", "ip_address": "1.1.1.1", "mac_address": null}]	["c4055356-df9b-4836-a142-49752a27f678"]	[{"id": "c2df64b0-595e-4cc3-82e3-84c75dd36ab3", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]	{"type": "System"}	null	2025-10-28 01:23:57.923299+00	2025-10-28 01:23:57.930352+00
e77e750f-6a0b-4b01-b253-0a5b3aa9e539	fa7742a1-5f18-4e4d-a336-0935a803df22	Google.com	google.com	Google.com	{"type": "ServiceBinding", "config": "f2da785c-ed22-4cc8-84b7-fdf653020d4c"}	[{"id": "02954440-7db2-4052-a89b-4dda04b55a56", "name": "Internet", "subnet_id": "29b0bf4c-957a-410d-9b35-e1b2c620e3ec", "ip_address": "203.0.113.53", "mac_address": null}]	["49de993d-5a2e-4354-8c89-d83c7e59467f"]	[{"id": "1d8daf2f-80b3-4b30-b15f-686bbf851021", "type": "Https", "number": 443, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-28 01:23:57.923342+00	2025-10-28 01:23:57.932851+00
7cfad113-26fa-405e-90db-472041e8cc6e	fa7742a1-5f18-4e4d-a336-0935a803df22	Mobile Device	\N	A mobile device connecting from a remote network	{"type": "ServiceBinding", "config": "bb474609-3d72-4e89-8287-9a5b99bb2704"}	[{"id": "c2285a9c-7b4a-42a9-ba09-d15a425ad86f", "name": "Remote Network", "subnet_id": "8dca18ea-f785-42cb-b85d-3eeea66f28c5", "ip_address": "203.0.113.201", "mac_address": null}]	["d4825e2f-0263-452d-8ea4-11d9475c7ef1"]	[{"id": "869410f6-45c5-4604-91d4-2142b4d0675c", "type": "Custom", "number": 0, "protocol": "Tcp"}]	{"type": "System"}	null	2025-10-28 01:23:57.923345+00	2025-10-28 01:23:57.935095+00
3f987eba-ac16-4aed-b0ac-dacd62a11141	fa7742a1-5f18-4e4d-a336-0935a803df22	NetVisor Server API	netvisor-server-1.netvisor_netvisor-dev	\N	{"type": "Hostname"}	[{"id": "b017c927-f0db-443b-b9c4-44507ae1ecf7", "name": null, "subnet_id": "5d179ca0-0932-479d-a30f-2413a9feaed3", "ip_address": "172.25.0.3", "mac_address": "7A:7D:70:DF:80:AC"}]	["51f3686b-398e-4621-9edc-1811af28e7d7"]	[{"id": "cb6f1a5c-76f7-431d-8c15-5bb160b6f953", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T01:26:52.096093713Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "Network"}]}	null	2025-10-28 01:26:52.096136+00	2025-10-28 01:27:01.307984+00
c7a127e3-5b88-4e39-a5f6-00e2dd790e0d	fa7742a1-5f18-4e4d-a336-0935a803df22	b15c43ce5e2f	b15c43ce5e2f	NetVisor daemon	{"type": "Hostname"}	[{"id": "7ad532bf-8b79-4d46-b6d5-58c19710327a", "name": "eth0", "subnet_id": "5d179ca0-0932-479d-a30f-2413a9feaed3", "ip_address": "172.25.0.4", "mac_address": "02:CD:A8:B8:6D:C7"}]	["f185a144-e77c-41d7-b493-1af9011b365f", "8e091f04-e738-4fe7-8bc4-4efc7f8e3899"]	[{"id": "33bc037a-5b7e-4399-979b-f802e580d990", "type": "Custom", "number": 60073, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T01:27:01.297198301Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "Network"}, {"date": "2025-10-28T01:26:44.233076126Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "SelfReport"}]}	null	2025-10-28 01:26:44.233077+00	2025-10-28 01:27:20.169669+00
17dad144-60b4-485c-b8b0-53043f490b59	fa7742a1-5f18-4e4d-a336-0935a803df22	Home Assistant	homeassistant-discovery.netvisor_netvisor-dev	\N	{"type": "Hostname"}	[{"id": "02d9c3a8-5556-44e9-b762-f00289356581", "name": null, "subnet_id": "5d179ca0-0932-479d-a30f-2413a9feaed3", "ip_address": "172.25.0.5", "mac_address": "C6:A3:A5:DA:F7:A8"}]	["533a08d9-7dbc-487a-a323-452791d06a7c"]	[{"id": "7053db57-09d8-4656-9575-a7c778647a6f", "type": "Custom", "number": 8123, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T01:27:01.298222801Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "Network"}]}	null	2025-10-28 01:27:01.298224+00	2025-10-28 01:27:20.199194+00
2b827df9-1878-4d34-970c-8c143dc64e99	fa7742a1-5f18-4e4d-a336-0935a803df22	Home Assistant	\N	\N	{"type": "None"}	[{"id": "2a7e2663-0d95-4bab-89fa-f8815b573efb", "name": null, "subnet_id": "5d179ca0-0932-479d-a30f-2413a9feaed3", "ip_address": "172.25.0.1", "mac_address": "B2:63:24:D7:1B:11"}]	["622e0404-300b-4ca1-88dd-29d137129e4a", "019f8f36-fb82-47f4-9c10-bab99b3205e3"]	[{"id": "1fbe538a-04a2-48bf-891f-3939f079ae3b", "type": "Custom", "number": 8123, "protocol": "Tcp"}, {"id": "f961ef57-9065-4f41-9faf-f50dfed7f85b", "type": "Custom", "number": 60072, "protocol": "Tcp"}]	{"type": "Discovery", "metadata": [{"date": "2025-10-28T01:27:11.096284097Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "Network"}]}	null	2025-10-28 01:27:11.096285+00	2025-10-28 01:27:20.215067+00
\.


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.networks (id, name, created_at, updated_at, is_default, user_id) FROM stdin;
fa7742a1-5f18-4e4d-a336-0935a803df22	My Network	2025-10-28 01:23:57.892944+00	2025-10-28 01:23:57.892945+00	t	7f79735c-209a-4e62-bc9d-c5f8dbabbc37
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, network_id, created_at, updated_at, name, host_id, bindings, service_definition, virtualization, source) FROM stdin;
c4055356-df9b-4836-a142-49752a27f678	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:23:57.923339+00	2025-10-28 01:23:57.92991+00	Cloudflare DNS	e1abd7cf-d901-40b1-b66e-9618b9ba7a12	[{"id": "434472b0-6d22-48be-8e12-23e86fc052c6", "type": "Port", "port_id": "c2df64b0-595e-4cc3-82e3-84c75dd36ab3", "interface_id": "8d9ed935-bbf8-4466-a89b-d574a35eb1f5"}]	"Dns Server"	null	{"type": "System"}
49de993d-5a2e-4354-8c89-d83c7e59467f	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:23:57.923343+00	2025-10-28 01:23:57.932578+00	Google.com	e77e750f-6a0b-4b01-b253-0a5b3aa9e539	[{"id": "f2da785c-ed22-4cc8-84b7-fdf653020d4c", "type": "Port", "port_id": "1d8daf2f-80b3-4b30-b15f-686bbf851021", "interface_id": "02954440-7db2-4052-a89b-4dda04b55a56"}]	"Web Service"	null	{"type": "System"}
d4825e2f-0263-452d-8ea4-11d9475c7ef1	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:23:57.923345+00	2025-10-28 01:23:57.934807+00	Mobile Device	7cfad113-26fa-405e-90db-472041e8cc6e	[{"id": "bb474609-3d72-4e89-8287-9a5b99bb2704", "type": "Port", "port_id": "869410f6-45c5-4604-91d4-2142b4d0675c", "interface_id": "c2285a9c-7b4a-42a9-ba09-d15a425ad86f"}]	"Client"	null	{"type": "System"}
622e0404-300b-4ca1-88dd-29d137129e4a	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:27:17.399077+00	2025-10-28 01:27:20.184287+00	Home Assistant	2b827df9-1878-4d34-970c-8c143dc64e99	[{"id": "f6b814b3-5b57-4701-b0fd-cedaa499f488", "type": "Port", "port_id": "1fbe538a-04a2-48bf-891f-3939f079ae3b", "interface_id": "2a7e2663-0d95-4bab-89fa-f8815b573efb"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T01:27:17.399061836Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "Network"}]}
51f3686b-398e-4621-9edc-1811af28e7d7	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:26:59.539226+00	2025-10-28 01:27:01.307357+00	NetVisor Server API	3f987eba-ac16-4aed-b0ac-dacd62a11141	[{"id": "2dcd1ecf-0438-4230-818b-d5a60fa6ea1c", "type": "Port", "port_id": "cb6f1a5c-76f7-431d-8c15-5bb160b6f953", "interface_id": "b017c927-f0db-443b-b9c4-44507ae1ecf7"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.3:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T01:26:59.539208050Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "Network"}]}
f185a144-e77c-41d7-b493-1af9011b365f	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:26:44.233082+00	2025-10-28 01:27:20.168354+00	NetVisor Daemon API	c7a127e3-5b88-4e39-a5f6-00e2dd790e0d	[{"id": "44f9b894-b705-4232-8366-b6f0ba02f855", "type": "Port", "port_id": "33bc037a-5b7e-4399-979b-f802e580d990", "interface_id": "7ad532bf-8b79-4d46-b6d5-58c19710327a"}]	"NetVisor Daemon API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": ["Updated match data on 2025-10-28 01:27:01.297394301 UTC", [{"data": "Response from http://172.25.0.4:60073/api/health contained \\"netvisor\\"", "type": "reason"}, {"data": "NetVisor Daemon self-report", "type": "reason"}]], "type": "container"}, "confidence": "Certain"}, "metadata": [{"date": "2025-10-28T01:27:01.297394301Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "Network"}, {"date": "2025-10-28T01:26:44.233082251Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "SelfReport"}]}
533a08d9-7dbc-487a-a323-452791d06a7c	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:27:08.068351+00	2025-10-28 01:27:20.198706+00	Home Assistant	17dad144-60b4-485c-b8b0-53043f490b59	[{"id": "9173651e-4db8-446d-acc8-e7f3fbf5293f", "type": "Port", "port_id": "7053db57-09d8-4656-9575-a7c778647a6f", "interface_id": "02d9c3a8-5556-44e9-b762-f00289356581"}]	"Home Assistant"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.5:8123/auth/authorize contained \\"home assistant\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T01:27:08.068304429Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "Network"}]}
019f8f36-fb82-47f4-9c10-bab99b3205e3	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:27:18.342978+00	2025-10-28 01:27:20.214417+00	NetVisor Server API	2b827df9-1878-4d34-970c-8c143dc64e99	[{"id": "985b1981-df11-41c4-8738-a9566fb5da47", "type": "Port", "port_id": "f961ef57-9065-4f41-9faf-f50dfed7f85b", "interface_id": "2a7e2663-0d95-4bab-89fa-f8815b573efb"}]	"NetVisor Server API"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Response from http://172.25.0.1:60072/api/health contained \\"netvisor\\"", "type": "reason"}, "confidence": "High"}, "metadata": [{"date": "2025-10-28T01:27:18.342964336Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "Network"}]}
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, network_id, created_at, updated_at, cidr, name, description, subnet_type, source) FROM stdin;
29b0bf4c-957a-410d-9b35-e1b2c620e3ec	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:23:57.923266+00	2025-10-28 01:23:57.923266+00	"0.0.0.0/0"	Internet	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).	"Internet"	{"type": "System"}
8dca18ea-f785-42cb-b85d-3eeea66f28c5	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:23:57.923268+00	2025-10-28 01:23:57.923268+00	"0.0.0.0/0"	Remote Network	This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend's networks, public WiFi, etc.).	"Remote"	{"type": "System"}
5d179ca0-0932-479d-a30f-2413a9feaed3	fa7742a1-5f18-4e4d-a336-0935a803df22	2025-10-28 01:26:44.226277+00	2025-10-28 01:26:44.226277+00	"172.25.0.0/28"	172.25.0.0/28	\N	"Lan"	{"type": "Discovery", "metadata": [{"date": "2025-10-28T01:26:44.226274751Z", "daemon_id": "2079500c-2e83-4176-9969-b7de96b2bc9d", "discovery_type": "SelfReport"}]}
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, created_at, updated_at) FROM stdin;
7f79735c-209a-4e62-bc9d-c5f8dbabbc37	Default Username	2025-10-28 01:23:57.89238+00	2025-10-28 01:23:57.892383+00
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

\unrestrict MV3NY8WgL1UFlvKW6bYyBYrNEdo0HTiizKt11yhSEyxghvly8JkJtCFgGilMLUJ

