# A2DP for Windows

Windows PC의 오디오를 Bluetooth 헤드폰으로 보내고 코덱을 제어하는 Rust 중심 프로젝트입니다.
A2DP는 Advanced Audio Distribution Profile이며, 이 프로젝트는 송신 측인 **Source**를 목표로 합니다.

> 현재는 설계와 OS 독립 Rust 기반 단계입니다. 설치 가능한 드라이버, 실제 오디오 전송, 동작이 검증된 코덱은 없습니다.

## 목표

- Windows 11 x64와 선정한 어댑터·헤드폰에서 SBC 재생 경로 검증
- LDAC, aptX, aptX HD 순으로 코덱 확장 검토
- AAC와 aptX Low Latency는 구현·권리 조건 검증 후 도입 판단
- 장치별 설정, 연결 복구, 절전 복귀, 진단과 기본 드라이버 복구 경로 제공

코덱 이름은 지원 목표입니다. Bluetooth 헤드폰의 지원 능력, 전송 경로, 구현 상태와 배포 조건이 모두 충족되어야 사용할 수 있습니다. HFP 마이크와 LE Audio는 초기 범위에 포함하지 않습니다.

## 현재 구성

| 경로 | 역할 |
| --- | --- |
| [AGENTS.md](AGENTS.md) | 작업·검증·Git·이슈·PR 공통 규칙 |
| [CLAUDE.md](CLAUDE.md) | 공통 지침 import |
| [.github](.github) | 결함·개선·부모·질문 이슈 폼과 PR 양식 |
| [crates/a2dp-core](crates/a2dp-core) | `no_std`, 외부 의존성 없는 코덱 후보 선택 정책과 경계 테스트 |
| [scripts](scripts) | 문서·양식·Rust 통합 검사 |
| [설계 문서](docs/README.md) | 요구사항, 아키텍처, 인터페이스, ADR, 구현·실기 검증 계획 |
| [UI 설계](docs/ui-design.md) | 장치·코덱 설정·오류 복구 화면, 상태별 동작과 접근성 |
| [장치 설정 시안](docs/ui/device-settings.png) | 왼쪽 기기 목록, 오른쪽 코덱·음질·버퍼·연결 설정 |
| [협업 구성](docs/project-conventions.md) | 작업 규칙, 이슈·PR 양식과 저장소 운영 기준 |
| [CHANGELOG.md](CHANGELOG.md) | 단계별 변경 이력 |

상세 구조는 [아키텍처](docs/architecture.md), 다음 구현 단계는 [로드맵](docs/roadmap.md)을 따릅니다. `a2dp-core`는 synthetic capability로 정책을 검증하며 장치를 조회하거나 재생하지 않습니다. 저장소 자체의 배포 라이선스는 아직 정하지 않았으며, 외부 코덱 소스·바이너리는 포함하지 않습니다.

## 개발 시작

Rustup, Python 3.11 이상, PowerShell 7 이상이 필요합니다. Windows에서는 MSVC C++ Build Tools와 Windows SDK를 준비합니다. 일반 core 검사에는 WDK가 필요하지 않습니다. Rust는 [rust-toolchain.toml](rust-toolchain.toml)의 1.99.0으로 고정합니다.

저장소 루트에서:

```powershell
python -m pip install -r scripts/requirements-checks.txt
pwsh -NoProfile -File scripts/check.ps1
```

Rust 정책 테스트만 실행하려면:

```powershell
cargo test --workspace --locked
```

현재 build 결과는 library입니다. 사용자용 실행 파일이나 `.sys`를 생성하지 않습니다. [개발 환경](docs/development.md)에 명령별 범위와 driver 준비 조건을 정리했습니다.

## 기여와 검증

기본 브랜치는 `main`입니다. 별도 `feature/` 브랜치와 워크트리에서 작업하고 PR 단위로 검증·머지합니다. 커밋 이력을 보존하기 위해 merge commit을 사용합니다.

[CI](.github/workflows/ci.yml)는 Windows·Linux에서 같은 검사 스크립트를 실행합니다. OS 독립 테스트 통과는 Windows driver·헤드폰 실기 검증과 구분합니다. 검증 범위와 남은 제약은 [검증 계획](docs/validation.md)을 따릅니다.
