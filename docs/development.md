# 개발 환경과 실행 범위

## 현재 단계

`crates/a2dp-core`는 `no_std`의 순수 codec 후보 정책 library입니다. 외부 Cargo dependency가 없으며, 실제 encoder·앱·장치 조회·driver는 없습니다. 일반 Rust 빌드와 Windows/Linux CI가 검사하는 범위도 이 library와 저장소 문서입니다.

## 준비와 실제 검사 명령

1. Rustup을 준비한다. 저장소의 [toolchain 파일](../rust-toolchain.toml)이 Rust 1.99.0, rustfmt, clippy를 선택한다.
2. Windows에는 MSVC C++ Build Tools와 Windows SDK를 준비한다. 일반 library 테스트에는 WDK가 필요 없다.
3. Python 3.11 이상과 PowerShell 7 이상을 준비한다. 프로젝트 기본 검사에 PyYAML 6.0.3을 사용한다.
4. 저장소 루트에서 아래 명령을 실행한다. CI도 같은 스크립트를 사용한다.

```powershell
python -m pip install -r scripts/requirements-checks.txt
pwsh -NoProfile -File scripts/check.ps1
```

| 명령 | 검사·생성 결과 |
| --- | --- |
| `python scripts/check_repository.py` | Git에 포함되거나 ignore되지 않은 소스의 UTF-8/LF, inline 상대 링크 대상, fence, YAML/TOML, form ID·PR 필수 항목 |
| `pwsh -NoProfile -File scripts/check.ps1 -Mode Quick` | 저장소 검사, diff, rustfmt, clippy |
| `pwsh -NoProfile -File scripts/check.ps1` | Quick + Rust tests/doctest, release library build, rustdoc |
| `cargo test --workspace --locked` | 12개 정책 테스트와 1개 문서 예제 |
| `cargo build --workspace --release --locked` | `target/release/`의 library, 실행 파일·driver 없음 |
| `cargo doc --workspace --no-deps --locked` | `target/doc/`의 Rust API 문서 |

저장소 검사는 inline Markdown 외부 링크의 공식 도메인·경로 허용 범위를 검사하지만 외부 URL의 실제 응답·Markdown anchor·GitHub 서버 전체 schema를 검증하지 않습니다. 정책 exhaustive 테스트는 64×64 capability 조합에서 반환한 후보가 양쪽에 존재하는지 확인하며 실제 장비 목록을 사용하지 않습니다. `Codec::ALL`은 목표 identity 목록으로 local encoder inventory가 아닙니다.

`Cargo.lock`을 커밋하고 CI는 `--locked`를 사용합니다. toolchain 갱신은 별도 변경으로 로컬·CI를 확인합니다. Cargo publish는 비활성화되어 있습니다. 실제 재생 검증은 [검증 계획](validation.md)을 따릅니다.

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
