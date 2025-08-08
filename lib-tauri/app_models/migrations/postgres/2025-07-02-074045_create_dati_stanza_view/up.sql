create or replace view v_mat_min_eff_stanza as
select si.stanza_id, mi.materiale
from stanza as s
         join stanza_con_infissi as si on s.id = si.stanza_id
         join infisso as i on si.infisso_id = i.id
         join materiale_infisso as mi on i.materiale = mi.materiale
where mi.eff_energetica in (select min(mi2.eff_energetica)
                            from stanza_con_infissi as si
                                     join infisso as i2 on si.infisso_id = i2.id
                                     join materiale_infisso as mi2 on i2.materiale = mi2.materiale
                            where si.stanza_id = s.id)
group by si.stanza_id, mi.materiale;

create or replace view v_vet_min_eff_stanza as
select si.stanza_id, mi.vetro
from stanza as s
         join stanza_con_infissi as si on s.id = si.stanza_id
         join infisso as i on si.infisso_id = i.id
         join vetro_infisso as mi on i.vetro = mi.vetro
where mi.eff_energetica in (select min(mi2.eff_energetica)
                            from stanza_con_infissi as si
                                     join infisso as i2 on si.infisso_id = i2.id
                                     join vetro_infisso as mi2 on i2.vetro = mi2.vetro
                            where si.stanza_id = s.id)
group by si.stanza_id, mi.vetro;

create or replace view v_mq_infissi as
select si.stanza_id, sum(i.mq * si.num_infisso) as mq_infissi
from infisso as i
         join stanza_con_infissi as si on i.id = si.infisso_id
group by si.stanza_id;

create or replace view v_dati_stanze as
select s.id,
       e.fascicolo,
       s.edificio_id                                  as chiave,
       s.piano,
       s.id_spazio,
       s.cod_stanza,
       s.destinazione_uso,
       s.altezza,
       s.spessore_muro,
       s.riscaldamento,
       s.raffrescamento,
       s.illuminazione,
       coalesce(round(dgs.mq_infissi::numeric, 2), 0)::float as mq_infissi,
       dgs.materiale,
       dgs.vetro
from stanza as s
         join edificio as e on e.chiave = s.edificio_id
    -- aggiungo i dati degli infissi alle stanze che c'è li hanno
         left join (
    -- recupero i dati delle stanze che hanno degli infissi
    select mq.stanza_id, mq.mq_infissi, m.materiale, v.vetro
    from v_mq_infissi as mq
             join v_mat_min_eff_stanza as m
                  on mq.stanza_id = m.stanza_id
             join v_vet_min_eff_stanza as v on m.stanza_id = v.stanza_id) as dgs
                   on s.id = dgs.stanza_id;
