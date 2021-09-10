DROP TABLE IF EXISTS evm_contracts;
CREATE TABLE evm_contracts (
   tx        bytea,
   network   bigint NOT NULL,
   blockhash bytea NOT NULL,
   blocknum  bigint NOT NULL,
   tm        timestamp without time zone NOT NULL,
   a         bytea NULL,
   input     bytea NULL,
   PRIMARY KEY (tx, network)
);
CREATE INDEX ON evm_contracts (a);
CREATE INDEX ON evm_contracts (cat, tm);

DROP TABLE IF EXISTS evm_contract_log;
CREATE TABLE evm_contract_log (
   tx     bytea,
   logindex int NOT NULL, 
   tm         timestamp without time zone NOT NULL,
   blocknum   bigint NOT NULL,
   a    bytea,
   topic      bytea,
   txdata     bytea,
   PRIMARY KEY (tx, logindex)
);
CREATE INDEX ON evm_contract_log (a, tm);