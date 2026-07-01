# (251015) 개발자의 99%는 PostgreSQL을 이해하지 못합니다 | The Coding Gopher
- https://youtu.be/P8rrhZTPEAQ?si=X_XkEfPnPmebYwrY

<hr />

# GeekNews에 나온 자료

- [230718_PostgreSQL 14 Internal 최종본 (postgrespro.com)Geeknews](https://news.hada.io/topic?id=9864&utm_source=weekly&utm_medium=email&utm_campaign=202330)
  - PostgreSQL 14의 내부구조(스냅샷, 버퍼캐시, WAL, 잠금, 질의 실행, 각종 색인)에 대해 소개한 무료 이북의 최종본이 지난 3월 즈음에 나왔길래 뒤늦게나마 소개해 봅니다.
    - PDF 다운로드 : https://edu.postgrespro.com/postgresql_internals-14_en.pdf

- [(외부링크)https://neon.tech/postgresql/postgresql-getting-started/postgresql-sample-database](https://neon.tech/postgresql/postgresql-getting-started/postgresql-sample-database)


# 최신자료

# (260701) **[Postgres 19의 새로운 점: 베타 릴리스 심층 분석](<https://news.hada.io/topic?id=30982&utm_source=discord&utm_medium=bot&utm_campaign=5116>)**
- 대규모 프로덕션 데이터베이스를 위한 **`REPACK CONCURRENTLY`** 가 코어에 내장되고, SQL 속성 그래프 쿼리·논리적 복제·VACUUM·성능 등 전 영역에 걸친 폭넓은 개선이 담긴 베타 릴리스  
- 파티션 **병합·분할** 지원과 시퀀스 값 동기화로 운영 중 설계 변경과 데이터 이동이 한층 수월해짐  
- 오토배큠에 **병렬 워커**와 우선순위 점수 체계가 도입되어 유지보수 작업의 처리량과 가시성 향상  
- **SQL/PGQ** 도입으로 관계형 모델을 버리지 않고 기존 데이터를 그래프 형태로 조회 가능  
- 단일 헤드라인 기능이 아닌 *…