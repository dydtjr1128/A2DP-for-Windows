# Starter kit 적용 기록

기준일: 2026-10-03 (KST)

원본: [dydtjr1128/project-starter-kit](https://github.com/dydtjr1128/project-starter-kit/tree/e759f1c3c4603b80044d8ed4b870f18de18a9486)

고정 revision: `e759f1c3c4603b80044d8ed4b870f18de18a9486`

## 선택과 조정

| 원본 (`templates/common/` 기준) | 적용 | 프로젝트별 조정 |
| --- | --- | --- |
| `AGENTS.md` | 루트 | 목적·Windows/Rust 환경·검사·문서·원격 작업 규칙, 드라이버/FFI/진단 정보 경계 |
| `CLAUDE.md` | 루트 | `@AGENTS.md` 단일 import 유지 |
| `.gitattributes` | 루트 | 텍스트 LF, Windows 배치 파일 CRLF, 이미지 binary |
| `.github/ISSUE_TEMPLATE/bug.yml` | 같은 경로 | OS 빌드, 어댑터·헤드폰·코덱·절전·기본 드라이버 비교 항목 |
| `.github/ISSUE_TEMPLATE/change.yml` | 같은 경로 | 현재/목표 상태와 완료 조건 유지 |
| `.github/ISSUE_TEMPLATE/parent.yml` | 같은 경로 | 오디오·전송·코덱·안정성 등 통합 결과를 관리할 때 사용 |
| `.github/ISSUE_TEMPLATE/question.yml` | 같은 경로 | 저장소 내 사용 문의 접수 |
| `.github/ISSUE_TEMPLATE/config.yml` | 같은 경로 | 빈 이슈 비허용, 존재하지 않는 외부 문의 링크 제거 |
| `.github/pull_request_template.md` | 같은 경로 | 한국어 요약·변경·완료 조건·검증·문제·제외 범위 유지 |

`.editorconfig`, `.gitignore`, README, 변경 이력은 이 프로젝트용으로 작성했습니다.
라벨·담당자·브랜치 보호·유료 서비스·서명·릴리스 자동화는 템플릿 도입으로 자동 설정하지 않습니다.
CI는 실제 검사 대상과 재현 가능한 명령이 생기는 Rust 기반 단계에서 추가합니다.

## 운영 기준

- 새 저장소에 덮어쓸 기존 파일이 없음을 확인한 뒤 적용했습니다.
- 커밋 메시지는 한국어 Conventional Commit, 브랜치는 `feature/`, merge commit으로 단계별 커밋을 보존합니다.
- 직접 요청은 불필요한 추적 이슈를 만들지 않고 PR에 `없음: 사용자 직접 요청`으로 기록합니다.
- 템플릿 원본에 프로젝트 라이선스를 대신 선택하는 권한은 없습니다. 이 저장소와 외부 코드의 라이선스는 각각 결정합니다.
- 원본 갱신은 이 revision과 새 revision의 diff를 먼저 비교하고 프로젝트별 문안을 보존합니다.

## 적용 확인

로컬에서 YAML 파싱, 필수 form ID·label·options, 문서 상대 링크, diff 공백을 확인합니다.
기본 브랜치 반영 뒤 GitHub의 이슈 선택 화면과 PR 본문이 실제 파일을 사용하는지 확인합니다.
양식 확인을 위해 공개 시험 이슈를 생성할 필요는 없습니다.
