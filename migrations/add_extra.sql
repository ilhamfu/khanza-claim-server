drop table if exists gmc_claim__user;
drop table if exists gmc_claim__docu_cache;

create table gmc_claim__user (
  id varchar(255) primary key,
  username varchar(255) unique not null,
  password varchar(255) not null,
  name varchar(255),
  must_seen BIT default 0,
  must_not_reject BIT default 0,
  must_accept BIT default 1
) ENGINE=InnoDB DEFAULT CHARACTER SET latin1 COLLATE latin1_swedish_ci;

create table gmc_claim__docu_cache (
  no_rawat varchar(8) primary key,
  value JSON,
  last_update timestamp not null default current_timestamp on update current_timestamp
) ENGINE=InnoDB DEFAULT CHARACTER SET latin1 COLLATE latin1_swedish_ci;

alter table gmc_claim__docu_cache add constraint fk__docu_cache__reg_periksa foreign key (no_rawat) references reg_periksa(no_rawat) on delete cascade;
