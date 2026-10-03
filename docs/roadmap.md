# 단계별 구현과 PR 계획

기간을 약속하는 일정표가 아니라 **증거가 쌓이는 순서**입니다. 각 단계는 선행 결과를 확인한 다음 진행합니다. M1의 driver binding·복구 경로가 실패하면 codec/UI 확장을 먼저 하지 않습니다.

## 이번 기반 작업

| 단계 | 브랜치 | 결과 | 검증 |
| --- | --- | --- | --- |
| 1 | `feature/project-conventions` | 협업 지침·양식, README, 변경 이력 | YAML·링크·diff, 원격 양식 |
| 2 | `feature/detailed-design` | 요구사항·상세 설계·ADR·실기 기준 | 출처·문서 정합성·링크·diff |
| 3 | `feature/rust-foundation` | OS 독립 Rust core와 실제 빌드·검사 기반 | Rust fmt/clippy/test/build, 문서 검사, Windows/Linux CI |

각 단계는 최신 `origin/main`에서 시작하고 PR 검증 뒤 merge commit으로 반영합니다. 직접 요청이므로 별도 관리 이슈를 강제로 만들지 않습니다. 처음 비어 있는 원격 저장소의 기준점에는 내용 없는 초기 커밋을 사용했습니다.

## M0: 재현 가능한 개발 기반

- 완료 결과: 협업 규칙·설계·고정 Rust toolchain과 lockfile, codec 후보 정책의 경계 테스트.
- UI 설계: 장치·연결 설정·진단·복구·트레이 화면과 접근성·상태 계약 정의. 앱 구현과 실기 증거는 별도.
- PR 단위: 문서/정책 기반과 검사 실행 경로가 함께 설명 가능한 크기로 분리.
- 증거: 로컬 및 CI의 실제 실행 결과.
- 제한: encoder, driver, 장치 조회, 실제 오디오와 kernel build는 별도 단계.

## M1: 가장 위험한 Windows 경로 검증

| 작업 | 선행 | 완료 증거 |
| --- | --- | --- |
| G1: 장비·baseline·복구 준비 | M0 | OS 빌드, radio/headphone/firmware, 기존 driver, 정상 기본 재생, 복구 절차 기록 |
| G2: toolchain·최소 driver build | G1 | Rust/WDK/SDK/VS exact version, 실제 driver package·load/unload, C bridge 필요성 |
| G3: device binding·L2CAP | G2 | 대상 노드 한정 binding, SDP·signaling/media channel·MTU·cancel, stock 복구 |
| G4: SBC test signal | G3 + SBC source 검토 | generated PCM → SBC → 실제 헤드폰, codec/frame 증거·10분 재생 |
| G5: audio endpoint spike | G2 | Windows render endpoint, PCM 순서·format·clock·서비스 소비 확인 |
| 통합 결정 | G3/G4/G5 | 선택 구조 ADR 갱신, 원래 audio 복구, 다른 Bluetooth 장치 보존 |

G4의 test tone은 Windows 앱 오디오 지원이 아닙니다. G5에서 소리가 endpoint에 들어왔다고 Bluetooth 전송이 증명되는 것도 아닙니다. **G3 binding과 rollback이 막히면 M1 미통과**입니다.

현재 호스트에 테스트 드라이버 설치, TESTSIGNING·Secure Boot 변경을 실행하는 단계는 이번 기반 작업에 포함하지 않습니다.

## M2: SBC로 Windows 앱 오디오 통합

- G4와 G5를 실제 bounded PCM/media pipeline으로 연결한다.
- `desired/proposed/active`, session generation, Stop/cancel/drain, 서비스 crash cleanup을 구현한다.
- 30분 지속 재생, 20회 연결 해제·재연결, 10회 절전·복귀를 선정 환경에서 수행한다.
- 장치 제거·encoder 실패·queue overrun·늦은 completion을 fault injection한다.
- 기준: BSOD 없음, 회수되지 않은 요청 없음, stale audio 없음, 기본 driver 복구 확인.
- 다음 진입: 로그·성능·오류 증거가 [검증 양식](validation.md)에 남아 있어야 한다.

## M3: 코덱과 조작 기능

1. LDAC backend·상세 협상·wire payload·실기 시험을 하나의 완결된 범위로 도입한다.
2. aptX와 aptX HD를 각각 같은 조건으로 도입한다.
3. 장치별 희망 설정의 atomic 저장과 실제 적용 결과를 분리한다.
4. [UI 상세 설계](ui-design.md)의 화면·상태·접근성 계약과 선택한 시안을 기준으로 UI 프레임워크를 검증하고 구현한다.
5. AAC·LL은 권리·구현 조건이 해결된 경우에만 독립 기능 단계로 편입한다.

한 codec의 통과를 다른 codec의 완료로 계산하지 않습니다. 코덱 선택 정책 변경은 unsupported 조합과 fallback 동작을 함께 검증합니다.

## M4: 호환성·안정성·배포 후보

- 어댑터·헤드폰 조합 확대, radio off/on, 다른 output으로 변경, HFP 전환, Windows 업데이트 시험
- 8시간 soak, 100회 재연결, 50회 sleep/resume를 정한 matrix에서 수행
- Driver Verifier, 전원/PnP, 메모리·핸들 증가, codec FFI fuzz와 cancellation stress
- 설치·업그레이드·제거·손상 artifact·서명 실패 복구 검증
- 지원 대상 선언, license/notice/source 제공, SIG 적용 확인, 서명·배포 체계

이후에만 “지원 장비”, “실제 지원 코덱”, “배포 가능”을 README와 릴리스에 표시합니다.

## 후속 작업 등록 기준

독립 완료 결과가 생기면 구현 이슈로 등록하고 공통 통합 결과가 필요할 때만 부모 이슈로 묶습니다. 위 표는 지금 시점의 계획이며 모든 행을 자동으로 공개 이슈로 만들지 않습니다. 새 PR은 관련 요구사항 ID, 검증 증거, 선행 통과 여부를 포함합니다.
