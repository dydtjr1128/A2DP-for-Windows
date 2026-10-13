# 설계 문서 지도

기준일: 2026-10-03 (KST). 상태: **초기 설계와 OS 독립 Rust 기반**, 실기 검증 전.

Windows PC의 오디오 전송과 사용자가 이해할 수 있는 기기별 설정을 목표로 독립 설계합니다. 개인 연구부터 시작하되 향후 배포 가능성도 검토합니다. 요구사항, 설계 판단, 구현 결과와 실기 증거를 구분하며 [공식 기술 근거](sources.md)를 별도로 관리합니다.

| 문서 | 답하는 질문 |
| --- | --- |
| [요구사항](requirements.md) | 누구에게 무엇을 제공하고 무엇으로 완료를 판단하는가 |
| [아키텍처](architecture.md) | Windows 오디오, Rust 서비스, 코덱과 Bluetooth를 어떻게 연결하는가 |
| [인터페이스 계약](interfaces.md) | 상태·버퍼·명령·실패·복구를 어떤 계약으로 연결하는가 |
| [UI 상세 설계](ui-design.md) | Bluetooth 목록·기기 설정·오류·복구·접근성을 화면과 행동으로 어떻게 표현하는가 |
| [장치 상태 계약](device-status.md) | 지원 codec·현재 format·전송률의 출처와 Windows/앱 경로를 어떻게 구분하는가 |
| [설계 리뷰 검증 기록](reviews/2026-10-03-design-review.md) | 해당 검토 시점의 지적·판단·미반영 보완안은 무엇인가 |
| [코덱·배포 조건](codec-and-distribution.md) | 어떤 구현을 검토하며 개인 실험과 배포를 어떻게 구분하는가 |
| [구현 순서](roadmap.md) | 다음 PR이 만들어야 할 결과와 실기 통과 조건은 무엇인가 |
| [검증 계획](validation.md) | 자동 검사와 실제 장비 시험으로 무엇을 증명하는가 |
| [개발 환경](development.md) | 현재 무엇을 실행할 수 있고 드라이버 준비에는 무엇이 필요한가 |
| [설계 결정 기록](adr/README.md) | 선택 근거, 대안과 재검토 조건은 무엇인가 |
| [협업 구성](project-conventions.md) | 협업 파일의 역할과 양식 검증 범위는 무엇인가 |

문서에서 **결정**은 이번 프로젝트가 선택한 원칙, **제안**은 검증할 구현 방법, **미확인**은 아직 증거가 없는 조건입니다. 설계상의 경로·API 이름은 별도 표기가 없으면 계획이며 현재 배포 API가 아닙니다.

현재 구현된 부분은 [a2dp-core](../crates/a2dp-core)의 codec identity와 후보 선택 정책, [검사 스크립트](../scripts/check.ps1)입니다. driver·encoder·service·IPC·UI와 실기 성능 목표는 계획 상태입니다.

## 담당 문서와 구현 확인

동작 규칙·값·상태의 정의는 담당 문서 한 곳에서 고치고, 요약·예시에는 그 문서를 연결합니다. 실제 동작은 코드와 실행 결과로 확인합니다. 문서와 구현이 다르면 어느 쪽을 변경할지 판단하고 불일치를 기록하며, 계획을 구현 완료로 바꾸지 않습니다.

| 변경 대상 | 담당 기준 | 함께 확인할 구현·검증 |
| --- | --- | --- |
| 코덱 후보 선택 | [core API와 구현](../crates/a2dp-core/src/codec.rs) | [정책 테스트](../crates/a2dp-core/tests/codec_policy.rs), 장비 capability와 분리된 순수 입력 |
| 프로세스·드라이버 경계 | [아키텍처](architecture.md) | [인터페이스](interfaces.md), [단계별 실기 조건](roadmap.md), 현재는 설계 |
| 명령·상태 전이·버퍼 수명 | [인터페이스](interfaces.md) | [검증 계획](validation.md), 현재 service·IPC 구현 없음 |
| 장치·코덱 상태와 측정값 | [장치 상태 계약](device-status.md) | [UI 설계](ui-design.md)의 표시·미확인 처리, 현재 실제 telemetry 없음 |
| 화면·조작·접근성 | [UI 설계](ui-design.md) | [HTML 시안](ui/device-settings.html), [시안 검증 기록](ui/design-qa.md), 모의 데이터와 실기 구분 |
| 개발·협업 절차 | [AGENTS.md](../AGENTS.md) | [검사 스크립트](../scripts/check.ps1), [CI](../.github/workflows/ci.yml), [이슈·PR 양식](../.github) |

ADR은 선택 근거와 재검토 조건, 리뷰 기록은 당시의 근거와 보완 제안을 보관합니다. 리뷰에 채택 대상으로 적혔더라도 담당 문서·구현에 반영되지 않은 제안은 현재 계약이 아닙니다.
