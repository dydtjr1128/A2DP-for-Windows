# 개발 환경과 실행 범위

## 현재 단계

저장소는 협업 규칙과 설계를 먼저 갖추는 단계입니다. 아직 실행 가능한 앱, encoder, 설치 가능한 driver가 없습니다. Rust 기반 PR에서 실제 manifest·검사 스크립트·CI를 추가한 뒤 이 문서의 명령을 갱신합니다.

현재 가능한 검사는 저장소 루트의 `git diff --check`, Markdown 상대 링크와 GitHub YAML 파싱입니다. 실제 재생 검증은 [검증 계획](validation.md)을 따릅니다.

## 일반 Rust 코드와 driver 환경 분리

| 구분 | 준비 | 검증 범위 |
| --- | --- | --- |
| 일반 Rust core | 고정 Rust toolchain, Windows MSVC linker 또는 Linux 개발 도구 | 순수 정책·값 검증·문서·unit test |
| Windows native bridge | Visual Studio C++ Build Tools, Windows SDK | 사용자 모드 native/FFI 경계 |
| Kernel driver | 호환되는 WDK/SDK/VS, upstream과 맞춘 driver Rust toolchain, WinDbg | driver build·sign·load·PnP·실기 |

일반 Cargo workspace의 toolchain을 WDK에 그대로 쓸 수 있다고 가정하지 않습니다. driver spike는 exact version과 환경을 기록하고 일반 workspace 밖에서 구성합니다. 설치 참고는 [WDK 다운로드](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk), [Rust driver upstream](https://github.com/microsoft/windows-drivers-rs)입니다.

## Git 작업 방식

- `origin/main` 갱신 후 목적별 `feature/` 브랜치와 워크트리에서 변경한다.
- diff·관련 검사를 완료한 후 커밋·push·PR을 진행한다.
- 후행 PR은 선행 PR merge 뒤 최신 main에서 시작하여 중복 diff를 피한다.
- 원격 base/head와 check 결과를 확인하고 merge commit으로 반영한다.
- 로컬 `main`을 fast-forward로 맞추고 깨끗한 상태인지 확인한다.

Git 계정·origin 인증 정보는 저장소 로컬 설정으로 관리합니다. 토큰이나 사용자 PC별 경로를 tracked script에 넣지 않습니다.

## 작업 범위 주의점

빌드·unit test는 Bluetooth 장치에 접근하거나 driver를 설치해서는 안 됩니다. 장치 binding, test certificate 설치, TESTSIGNING/Secure Boot, 관리자 service 설치는 M1 이후 별도의 실기 절차입니다. 검사 스크립트에 이러한 부수 효과를 숨기지 않습니다.

인증서 오류가 있는 웹 조회는 AGENTS.md의 `curl-cffi` 재시도 규칙을 따릅니다. 실제 timeout이 계속 발생한 URL만 미확인으로 남기며 일반 실패를 방화벽 문제로 단정하지 않습니다.
