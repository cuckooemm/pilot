USE pilot;

INSERT INTO department (`name`) VALUE ('System')
# add admin password 123456
INSERT INTO users (`account`, email, nickname, `password`, `dept_id`, `level`) VALUE ('admin','','admin','$2a$12$ACgenj7SIVj5q.5Zzij20ux52cz16ERiSQBOOq3F.qaPUBou6r9GC',1,100)