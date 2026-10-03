# 협업 구성

## 저장소 구성

| 구성 | 담당 역할 |
| --- | --- |
| `AGENTS.md` | 프로젝트 환경, 작업·검증·Git·이슈·PR 규칙의 단일 기준 |
| `CLAUDE.md` | 공통 지침 import |
| `.gitattributes` / `.editorconfig` | UTF-8·LF 기본값, Windows 배치 파일 CRLF |
| `.gitignore` | 생성 파일, 서명 키, 녹음·덤프·로컬 로그 제외 |
| `.github/ISSUE_TEMPLATE` | 결함·개선·부모·질문 양식 |
| `.github/pull_request_template.md` | 변경 결과, 완료 조건, 실제 검증과 남은 범위 |
| `.github/workflows/ci.yml` | Windows/Linux 저장소·Rust 검사 |

## 운영 기준

- 새 브랜치는 `feature/`, 커밋과 PR은 한국어 Conventional Commit을 사용한다.
- `main`에서 분리한 워크트리에서 작업하고, 최신 head의 검사를 확인한 뒤 merge commit으로 반영한다.
- 관련 이슈가 없는 직접 요청은 PR에 그 사유를 기록한다. 독립 완료 결과를 관리할 필요가 있을 때만 이슈로 등록한다.
- 결함에는 OS·어댑터·헤드폰·코덱·절전과 복구 조건을 남긴다. 공개 기록에는 원시 오디오와 장치 식별자를 넣지 않는다.
- 문서는 제품 요구사항과 공식 기술 근거를 중심으로 작성한다. 설계·시각 자료·예시 데이터는 이 프로젝트용으로 작성한다.
- 검사 명령과 실기 검증의 구분은 [개발 환경](development.md)과 [검증 계획](validation.md)을 따른다.
- 라벨·담당자·브랜치 보호·서명·배포·프로젝트 라이선스는 필요할 때 별도로 정한다.

## 양식 검증

로컬 검사는 YAML 구문, form ID 중복, 필수 속성과 PR 제목 항목을 검사합니다. 실제 GitHub 이슈 작성 화면은 로그인한 환경에서 별도로 확인합니다. 양식을 확인하기 위해 공개 시험 이슈를 생성하지 않습니다.
