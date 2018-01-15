CREATE TABLE category (
    name character varying(128) NOT NULL,
    abbr character(2) NOT NULL,
    description character varying(512) NOT NULL
);

CREATE TABLE category_link (
    category_id character varying(128) NOT NULL,
    required_id character varying(128) NOT NULL,
    level integer NOT NULL
);

CREATE TABLE component (
    name character varying(256) NOT NULL,
    description text NOT NULL,
    unit character varying(256) DEFAULT 'units'::character varying NOT NULL,
    cost real DEFAULT 0 NOT NULL,
    weight real DEFAULT 0 NOT NULL,
    volume real DEFAULT 0 NOT NULL
);

CREATE TABLE component_subset (
    component_id character varying(256) NOT NULL,
    subset_of character varying(256) NOT NULL
);

CREATE TABLE page (
    name character varying(512) NOT NULL,
    text text NOT NULL
);

CREATE TABLE spell (
    name character varying(512) NOT NULL,
    description text DEFAULT ''::text NOT NULL,
    range real DEFAULT '-1'::integer NOT NULL,
    casting_time interval DEFAULT '00:00:00'::interval NOT NULL,
    duration interval DEFAULT '00:00:00'::interval
);

CREATE TABLE spell_category (
    spell_id character varying(512) NOT NULL,
    category_id character varying(128) NOT NULL,
    level integer NOT NULL
);

CREATE TABLE spell_component (
    spell_id character varying(512) NOT NULL,
    component_id character varying(256) NOT NULL,
    notes text DEFAULT ''::text NOT NULL,
    quantity numeric
);

CREATE TABLE spell_produces (
    spell_id character varying(512) NOT NULL,
    component_id character varying(256) NOT NULL,
    notes text DEFAULT ''::text NOT NULL,
    quantity numeric
);


ALTER TABLE ONLY category_link
    ADD CONSTRAINT category_link_pkey PRIMARY KEY (category_id, required_id);

ALTER TABLE ONLY category
    ADD CONSTRAINT category_pkey PRIMARY KEY (name);

ALTER TABLE ONLY component
    ADD CONSTRAINT component_pkey PRIMARY KEY (name);

ALTER TABLE ONLY component_subset
    ADD CONSTRAINT component_subset_pkey PRIMARY KEY (component_id, subset_of);

ALTER TABLE ONLY page
    ADD CONSTRAINT page_pkey PRIMARY KEY (name);

ALTER TABLE ONLY spell_category
    ADD CONSTRAINT spell_category_pkey PRIMARY KEY (spell_id, category_id);

ALTER TABLE ONLY spell_component
    ADD CONSTRAINT spell_component_pkey PRIMARY KEY (spell_id, component_id);

ALTER TABLE ONLY spell
    ADD CONSTRAINT spell_pkey PRIMARY KEY (name);

ALTER TABLE ONLY spell_produces
    ADD CONSTRAINT spell_produces_pkey PRIMARY KEY (spell_id, component_id);

ALTER TABLE ONLY category_link
    ADD CONSTRAINT category_link_category_id_fkey FOREIGN KEY (category_id) REFERENCES category(name) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE ONLY category_link
    ADD CONSTRAINT category_link_required_id_fkey FOREIGN KEY (required_id) REFERENCES category(name) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE ONLY component_subset
    ADD CONSTRAINT component_subset_component_id_fkey FOREIGN KEY (component_id) REFERENCES component(name) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE ONLY component_subset
    ADD CONSTRAINT component_subset_subset_of_fkey FOREIGN KEY (subset_of) REFERENCES component(name) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE ONLY spell_category
    ADD CONSTRAINT spell_category_category_id_fkey FOREIGN KEY (category_id) REFERENCES category(name) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE ONLY spell_category
    ADD CONSTRAINT spell_category_spell_id_fkey FOREIGN KEY (spell_id) REFERENCES spell(name) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE ONLY spell_component
    ADD CONSTRAINT spell_component_component_id_fkey FOREIGN KEY (component_id) REFERENCES component(name) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE ONLY spell_component
    ADD CONSTRAINT spell_component_spell_id_fkey FOREIGN KEY (spell_id) REFERENCES spell(name) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE ONLY spell_produces
    ADD CONSTRAINT spell_produces_component_id_fkey FOREIGN KEY (component_id) REFERENCES component(name) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE ONLY spell_produces
    ADD CONSTRAINT spell_produces_spell_id_fkey FOREIGN KEY (spell_id) REFERENCES spell(name) ON UPDATE CASCADE ON DELETE CASCADE;