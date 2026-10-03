# A2DP for Windows

Windows PC의 오디오를 Bluetooth 헤드폰으로 보내고 코덱을 제어하는 Rust 중심 프로젝트입니다.
A2DP는 Advanced Audio Distribution Profile이며, 이 프로젝트는 송신 측인 **Source**를 목표로 합니다.

> 현재는 프로젝트 기반을 준비하는 단계입니다. 설치 가능한 드라이버, 실제 오디오 전송, 동작이 검증된 코덱은 없습니다.

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
| [템플릿 적용 기록](docs/starter-kit-adoption.md) | 원본 revision, 적용·조정 내역 |
| [CHANGELOG.md](CHANGELOG.md) | 단계별 변경 이력 |

상세 설계와 Rust 빌드 기반은 후속 단계에서 추가합니다. 저장소 자체의 배포 라이선스는 아직 정하지 않았으며, 외부 코덱 소스·바이너리는 포함하지 않습니다.

## 기여와 검증

기본 브랜치는 `main`입니다. 별도 `feature/` 브랜치와 워크트리에서 작업하고 PR 단위로 검증·머지합니다. 커밋 이력을 보존하기 위해 merge commit을 사용합니다.

현재 단계에서는 저장소 루트에서 `git diff --check`와 문서 링크·GitHub YAML 양식을 확인합니다. 앱 실행·드라이버 빌드 명령은 아직 없습니다.
