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

## 담당 기준

작업·Git·이슈 관계·라벨·우선순위 규칙은 [AGENTS.md](../AGENTS.md)에서 관리합니다. 이 문서는 협업 파일의 역할과 양식 검증 범위를 안내합니다. 검사 명령은 [개발 환경](development.md), 실기 통과 조건은 [검증 계획](validation.md), 동작별 담당 문서는 [문서 지도](README.md)를 따릅니다.

이슈 양식은 결함에 `bug`, 개선·부모에 `enhancement`, 문의에 `question`을 제안합니다. 우선순위 선택은 제안 기록이며 라벨을 자동 변경하지 않습니다. 담당자가 근거를 확인해 분류합니다. PR도 실제 변경에 해당하는 성격·보조 라벨을 사용합니다.

브랜치 보호, 드라이버 서명·배포와 프로젝트 라이선스는 별도 설정·판단 대상입니다. 협업 양식이나 로컬 검사 통과만으로 이 항목이 구성됐다고 보지 않습니다.

## 양식 검증

로컬 검사는 YAML 구문, form ID 중복, 필수 속성과 PR 제목 항목을 검사합니다. 폼의 기본 라벨이 실제 저장소에 있는지, 선택지가 GitHub에서 표시되는지는 원격 설정·작성 화면에서 별도로 확인합니다. 양식을 확인하기 위해 공개 시험 이슈를 생성하지 않습니다.
