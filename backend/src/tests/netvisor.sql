--
-- PostgreSQL database dump
--

\restrict 6WvGtv1tsdBMHn8ETP4xuOnoVe5t4QMxZdEAOh9qTFb6yKYq8O0KWITTZtCkAQP

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
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


--
-- Name: daemons; Type: TABLE; Schema: public; Owner: -
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


--
-- Name: groups; Type: TABLE; Schema: public; Owner: -
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


--
-- Name: hosts; Type: TABLE; Schema: public; Owner: -
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


--
-- Name: networks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.networks (
    id uuid NOT NULL,
    name text NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    is_default boolean NOT NULL,
    user_id uuid NOT NULL
);


--
-- Name: services; Type: TABLE; Schema: public; Owner: -
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


--
-- Name: subnets; Type: TABLE; Schema: public; Owner: -
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


--
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users (
    id uuid NOT NULL,
    name text NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


--
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public._sqlx_migrations VALUES (20251006215000, 'users', '2025-10-21 17:52:37.354237+00', true, '\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7', 3092791);
INSERT INTO public._sqlx_migrations VALUES (20251006215100, 'networks', '2025-10-21 17:52:37.358117+00', true, '\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3', 3145000);
INSERT INTO public._sqlx_migrations VALUES (20251006215151, 'create hosts', '2025-10-21 17:52:37.361674+00', true, '\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962', 2994709);
INSERT INTO public._sqlx_migrations VALUES (20251006215155, 'create subnets', '2025-10-21 17:52:37.365023+00', true, '\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926', 2712042);
INSERT INTO public._sqlx_migrations VALUES (20251006215201, 'create groups', '2025-10-21 17:52:37.36807+00', true, '\x96cdc35b7ad03869a836d4a4fe8c3060d075c32edce248827903ceab5c4e41b0727300d6c5755e54973f3ada9e50293a', 2455791);
INSERT INTO public._sqlx_migrations VALUES (20251006215204, 'create daemons', '2025-10-21 17:52:37.370856+00', true, '\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f', 2954584);
INSERT INTO public._sqlx_migrations VALUES (20251006215212, 'create services', '2025-10-21 17:52:37.374144+00', true, '\xe92885a5c8ea6bfa00c702c1aa81960a54704ab7219223ff469c3e6f2517ffe75dbed8bb4efd87de79fb3cf2c86e5c23', 2933959);


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- Data for Name: groups; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- Data for Name: hosts; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.hosts VALUES ('def483f2-3ddb-44f1-b5b8-0210e0d81671', 'b948ed99-7b49-42c6-a780-35955665f6a9', 'Cloudflare DNS', NULL, 'Cloudflare DNS', '{"type": "ServiceBinding", "config": "96db318d-33df-48c5-b12d-b3178b5d1323"}', '[{"id": "60412457-d6ac-44b8-8cd9-07da432b0b8c", "name": "Internet", "subnet_id": "ff290143-4c8f-4ac5-9ccf-750bff68a5cb", "ip_address": "1.1.1.1", "mac_address": null}]', '["a60fb7dc-7c5b-4397-9fbb-3fe679b7f505"]', '[{"id": "b05085ec-d168-4c66-a7ce-4ed37f783fd9", "type": "DnsUdp", "number": 53, "protocol": "Udp"}]', '{"type": "System"}', 'null', '2025-10-21 17:53:03.338486+00', '2025-10-21 17:53:03.352863+00');
INSERT INTO public.hosts VALUES ('80da4981-e82e-4837-9ad9-0c20a9c1ee7e', 'b948ed99-7b49-42c6-a780-35955665f6a9', 'Google.com', 'google.com', 'Google.com', '{"type": "ServiceBinding", "config": "dae127fa-8abc-47b1-b7c6-00f930db60ee"}', '[{"id": "5d7a6f11-2e6b-418b-b167-ff72ce1874d1", "name": "Internet", "subnet_id": "ff290143-4c8f-4ac5-9ccf-750bff68a5cb", "ip_address": "203.0.113.24", "mac_address": null}]', '["0750bb8c-93ee-4574-9841-d1d761e383bd"]', '[{"id": "9f1a63c0-810c-4a52-a55b-f044bc49385a", "type": "Https", "number": 443, "protocol": "Tcp"}]', '{"type": "System"}', 'null', '2025-10-21 17:53:03.338526+00', '2025-10-21 17:53:03.356967+00');
INSERT INTO public.hosts VALUES ('f03cda41-ba05-4a48-95b6-70ab03826c99', 'b948ed99-7b49-42c6-a780-35955665f6a9', 'Mobile Device', NULL, 'A mobile device connecting from a remote network', '{"type": "ServiceBinding", "config": "a014fe4c-4046-4889-bee6-e5fdbafa42bd"}', '[{"id": "815f4831-6117-475d-a3b4-16f249e791ab", "name": "Remote Network", "subnet_id": "bc9bd620-e94a-466d-8564-5521d7692e47", "ip_address": "203.0.113.125", "mac_address": null}]', '["373b1363-dc50-4109-aa98-703882835e9a"]', '[{"id": "e28ec6eb-88bc-47f2-8010-3993cce8643d", "type": "Custom", "number": 0, "protocol": "Tcp"}]', '{"type": "System"}', 'null', '2025-10-21 17:53:03.338531+00', '2025-10-21 17:53:03.360367+00');


--
-- Data for Name: networks; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.networks VALUES ('b948ed99-7b49-42c6-a780-35955665f6a9', 'My Network', '2025-10-21 17:53:03.290802+00', '2025-10-21 17:53:03.290805+00', true, 'f354bc63-cd7b-45fe-b7cf-323d9dd44ef9');


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.services VALUES ('a60fb7dc-7c5b-4397-9fbb-3fe679b7f505', 'b948ed99-7b49-42c6-a780-35955665f6a9', '2025-10-21 17:53:03.338522+00', '2025-10-21 17:53:03.338522+00', 'Cloudflare DNS', 'def483f2-3ddb-44f1-b5b8-0210e0d81671', '[{"id": "96db318d-33df-48c5-b12d-b3178b5d1323", "type": "Layer4", "port_id": "b05085ec-d168-4c66-a7ce-4ed37f783fd9", "interface_id": "60412457-d6ac-44b8-8cd9-07da432b0b8c"}]', '"Dns Server"', 'null', '[]', '[]', '{"type": "System"}');
INSERT INTO public.services VALUES ('0750bb8c-93ee-4574-9841-d1d761e383bd', 'b948ed99-7b49-42c6-a780-35955665f6a9', '2025-10-21 17:53:03.338528+00', '2025-10-21 17:53:03.338528+00', 'Google.com', '80da4981-e82e-4837-9ad9-0c20a9c1ee7e', '[{"id": "dae127fa-8abc-47b1-b7c6-00f930db60ee", "type": "Layer4", "port_id": "9f1a63c0-810c-4a52-a55b-f044bc49385a", "interface_id": "5d7a6f11-2e6b-418b-b167-ff72ce1874d1"}]', '"Web Service"', 'null', '[]', '[]', '{"type": "System"}');
INSERT INTO public.services VALUES ('373b1363-dc50-4109-aa98-703882835e9a', 'b948ed99-7b49-42c6-a780-35955665f6a9', '2025-10-21 17:53:03.338532+00', '2025-10-21 17:53:03.338532+00', 'Mobile Device', 'f03cda41-ba05-4a48-95b6-70ab03826c99', '[{"id": "a014fe4c-4046-4889-bee6-e5fdbafa42bd", "type": "Layer4", "port_id": "e28ec6eb-88bc-47f2-8010-3993cce8643d", "interface_id": "815f4831-6117-475d-a3b4-16f249e791ab"}]', '"Client"', 'null', '[]', '[]', '{"type": "System"}');


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.subnets VALUES ('ff290143-4c8f-4ac5-9ccf-750bff68a5cb', 'b948ed99-7b49-42c6-a780-35955665f6a9', '2025-10-21 17:53:03.338381+00', '2025-10-21 17:53:03.338381+00', '"0.0.0.0/0"', 'Internet', 'This subnet uses the 0.0.0.0/0 CIDR as an organizational container for services running on the internet (e.g., public DNS servers, cloud services, etc.).', '"Internet"', '{"type": "System"}');
INSERT INTO public.subnets VALUES ('bc9bd620-e94a-466d-8564-5521d7692e47', 'b948ed99-7b49-42c6-a780-35955665f6a9', '2025-10-21 17:53:03.338387+00', '2025-10-21 17:53:03.338387+00', '"0.0.0.0/0"', 'Remote Network', 'This subnet uses the 0.0.0.0/0 CIDR as an organizational container for hosts on remote networks (e.g., mobile connections, friend''s networks, public WiFi, etc.).', '"Remote"', '{"type": "System"}');


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.users VALUES ('f354bc63-cd7b-45fe-b7cf-323d9dd44ef9', '', '2025-10-21 17:53:03.28559+00', '2025-10-21 17:53:03.285594+00');


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: daemons daemons_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.daemons
    ADD CONSTRAINT daemons_pkey PRIMARY KEY (id);


--
-- Name: groups groups_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_pkey PRIMARY KEY (id);


--
-- Name: hosts hosts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_pkey PRIMARY KEY (id);


--
-- Name: networks networks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.networks
    ADD CONSTRAINT networks_pkey PRIMARY KEY (id);


--
-- Name: services services_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_pkey PRIMARY KEY (id);


--
-- Name: subnets subnets_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_pkey PRIMARY KEY (id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: idx_daemon_host_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_daemon_host_id ON public.daemons USING btree (host_id);


--
-- Name: idx_daemons_network; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_daemons_network ON public.daemons USING btree (network_id);


--
-- Name: idx_groups_network; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_groups_network ON public.groups USING btree (network_id);


--
-- Name: idx_hosts_network; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_hosts_network ON public.hosts USING btree (network_id);


--
-- Name: idx_services_host_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_services_host_id ON public.services USING btree (host_id);


--
-- Name: idx_services_network; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_services_network ON public.services USING btree (network_id);


--
-- Name: idx_subnets_network; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_subnets_network ON public.subnets USING btree (network_id);


--
-- Name: daemons daemons_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.daemons
    ADD CONSTRAINT daemons_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- Name: groups groups_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- Name: hosts hosts_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- Name: networks networks_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.networks
    ADD CONSTRAINT networks_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: services services_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_host_id_fkey FOREIGN KEY (host_id) REFERENCES public.hosts(id) ON DELETE CASCADE;


--
-- Name: services services_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- Name: subnets subnets_network_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_network_id_fkey FOREIGN KEY (network_id) REFERENCES public.networks(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict 6WvGtv1tsdBMHn8ETP4xuOnoVe5t4QMxZdEAOh9qTFb6yKYq8O0KWITTZtCkAQP

