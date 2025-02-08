# SQL_Training_PostgreSQL

# link

- 내가 정리한 PostgreSQL 커멘드Command 모음
  - https://github.com/YoungHaKim7/postgresql_gy



<hr />


# PostgreSQL Tutorial[|🔝|](#link)
- https://neon.tech/postgresql/tutorial


<hr />

# Docker로 PostgreSQL 설치[|🔝|](#link)
- https://xeppetto.github.io/%EC%86%8C%ED%94%84%ED%8A%B8%EC%9B%A8%EC%96%B4/WSL-and-Docker/15-Docker-PostGreSQL/

- https://judo0179.tistory.com/entry/Docker-Postgresql-%EC%84%A4%EC%B9%98-%EB%B0%8F-%EC%85%8B%ED%8C%85%ED%95%98%EA%B8%B0-1

```
docker run --name {name-of-container} -v {name-of-volume}:{volume-storage-location} -p {desired-port}:5432 -e POSTGRES_PASSWORD={desired-password} -d {desired-postgres-image}

docker run -p 5432:5432 --name postgres -e POSTGRES_PASSWORD=1q2w3e4r -d postgres


docker exec -it postgres /bin/bash




root@ac61c662ee4c:/# psql -U postgres
psql (13.0 (Debian 13.0-1.pgdg100+1))
Type "help" for help.

postgres=# CREATE USER seongwon PASSWORD '1q2w3e4r' SUPERUSER;
CREATE ROLE

postgres=# CREATE DATABASE test OWNER seongwon;
CREATE DATABASE

postgres=# \c test seongwon
You are now connected to database "test" as user "seongwon".
test=# CREATE TABLE star (
id integer NOT NULL,
name character varying(255),
class character varying(32),
age integer,
radius integer,
lum integer,
magnt integer,
CONSTRAINT star_pk PRIMARY KEY (id)
);
CREATE TABLE

test=# \dt
        List of relations
 Schema | Name | Type  |  Owner
--------+------+-------+----------
 public | star | table | seongwon
(1 row)
```

- https://medium.com/@bengiese22/how-to-run-postgresql-in-docker-on-mac-for-local-development-b7d79afd9219

<hr />

# PostgreSQL에 sql파일 넣기[|🔝|](#link)
- https://stackoverflow.com/questions/9736085/run-a-postgresql-sql-file-using-command-line-arguments


```
psql -h localhost -d userstoreis -U admin -p 5432 -a -q -f /home/jobs/Desktop/resources/postgresql.sql

-h PostgreSQL server IP address
-d database name
-U user name
-p port which PostgreSQL server is listening on
-f path to SQL script
-a all echo
-q quiet 
-f file

export PGPASSWORD=<password>
psql -h <host> -d <database> -U <user_name> -p <port> -a -w -f <file>.sql
```

<hr />

# PostgreSQL 설치 위치(`/bin/psql`)[|🔝|](#link)

- https://stackoverflow.com/questions/9736085/run-a-postgresql-sql-file-using-command-line-arguments

```
psql --version
which psql

Mine is version 9.1.6 located in /bin/psql.
```



<hr />

# SQL명령어 그림으로 이해하기[|🔝|](#link)
- 출처 : https://www.instagram.com/reel/DBrYJ_EhGku/?igsh=MWdwY2htemZ1b2xs
  - fork link: https://economiceco.tistory.com/19642

<img width=450px src="https://github.com/user-attachments/assets/0da8b29c-bad8-4f4e-9857-f683e336744e" />

<hr />

# SQL Databases vs NoSQL Databases[|🔝|](#link)

|SQL Databases|NoSQL Databases|
|-|-|
|1. PostgreSQL<br>2. MySQL<br>3. SQLite<br>4. SQL Server<br>5. Oracle<br>6. CockroachDB|1. Mongo DB<br>2. Redis<br>3. ElasticSearch<br>4. Firebase<br>5. Dynamo DB|

- 출처 : 13분9초 https://youtu.be/KBDSJU3cGkc?si=TcdAlhA0fWEC3VaE

<hr />

# PostgreSQL[|🔝|](#link)
- ebook
  - https://www.postgresqltutorial.com/
  - Tutorial모음
    - Learning SQL from scratch 🔴 PostgreSQL Live #1 | Xkonti
      - https://www.youtube.com/live/rYwXxc9Cpbo?si=4aP3wTCQbl1oItFN
    - PostgreSQL Tutorial for Beginners | freeCodeCamp.org
      -  https://youtu.be/SpfIwlAYaKk?si=cXVAdLMQQacjMnf_
    - [(220929)PostgreSQL Tutorial Full Course 2022 | Derek Banas](https://youtu.be/85pG_pDkITY?si=x8asVdoWzfnlFT0p)
    - [🐘 Hazel Bachrach라는 개발자가 Postgres에 대해 미리 알았더라면 좋았을 것들을 모아 정리했음. 공식 문서가 A4용지로 무려 3,200 페이지에 달하기 때문에 주니어 개발자에게는 굉장히 부담스러울 수밖에 없다고. 😱 데이터 정규화, psql 활용도 높이기, 인덱스 작동 방식, JSONB 사용 시 주의사항 등에 대해 적혀 있다. 물론 우리에게는 니꼬쌤의 SQL 강의가 있으니 안심해도 좋음!What I Wish Someone Told Me About Postgres Nov 11, 2024](https://challahscript.com/what_i_wish_someone_told_me_about_postgres)
  - 다른 Tutorial MySQL인듯
    - Database Engineering Complete Course | DBMS Complete Course | Nerd's lesson
      - https://youtu.be/iwRneX7GIGI?si=D6d409BipZM1ngXA



<hr>

# (230824)SQL For Web Developers - Complete Database Course | freeCodeCamp.org[|🔝|](#link)
- https://youtu.be/KBDSJU3cGkc?si=TcdAlhA0fWEC3VaE

<hr />

# SQL for Data Analytics - Learn SQL in 4 Hours | Luke Barousse[|🔝|](#link)
- https://youtu.be/7mz73uXD9DA?si=kKUsGBc8lKkEC6c2
  - VSCode로 PostgreSQL에 연결해서 연습하기
    - https://youtu.be/7mz73uXD9DA?si=kCdvh6U1JgA-RJRo&t=5778 

<hr />

# (241009)Databases In-Depth – Complete Course | freeCodeCamp.org[|🔝|](#link)
- https://youtu.be/pPqazMTzNOM?si=VwH-e-MRPX8LXxgl
