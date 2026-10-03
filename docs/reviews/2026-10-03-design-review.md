# 설계·구조 일반/적대적 리뷰 검증 기록

- 날짜: 2026-10-03 (KST)
- 리뷰 대상: `e577e875111fa63e07292ebf4ea39d4a9c1fc6e0`
- 변경 기준: `a7ed935f72db98df3b6fbfa73e7144e31758d231..e577e875111fa63e07292ebf4ea39d4a9c1fc6e0`
- 목적: Windows에서 이미 연결한 장치의 설정 UI, 현재 상태·지원 codec, Rust/Windows 경계와 설정 수명주기 검토
- 상태: 외부 의견을 아래와 같이 정적으로 재검증. 아래 보완안은 아직 설계 본문에 반영하지 않음

## 실행과 증거 범위

| 항목 | 일반 리뷰 | 적대적 리뷰 |
| --- | --- | --- |
| 요청 모델 | `claude-opus-5-5` | `claude-opus-5-5` |
| 실행 | 1회, 15분 상한 | 1회, 15분 상한 |
| CLI 결과 | success, is_error=false | success, is_error=false |
| 기록된 실행 시간 | 113,592 ms | 230,878 ms |
| 실제 modelUsage | `claude-opus-5-5` | `claude-opus-5-5`, `claude-fable-5-1` |
| 원문 | [일반 리뷰](2026-10-03-ordinary.md) | [적대적 리뷰](2026-10-03-adversarial.md) |

두 실행 모두 Opus 5.5를 지정했다. 적대적 실행에는 추가 모델 사용이 기록되어 있어 **Opus 단독 리뷰로 확정하지 않는다**. 원인은 이 결과 JSON과 빈 stderr만으로 확인할 수 없으며, 모델을 변경하거나 재시도한 별도 실행은 하지 않았다.

범위는 architecture, interfaces, device-status, UI 문서·시안, requirements, roadmap, validation, codec-and-distribution, ADR 0001~0003 및 직접 근거인 README, sources, codec 정책 코드와 테스트로 한정했다. reviewer는 정적 읽기만 요청받았고 파일 수정·실행 검증·네트워크·추가 reviewer는 금지했다. permission_denials는 양쪽 모두 비어 있었다. 외부 결과의 일부 문서는 키워드 검색 또는 미열람으로 남았으므로 전 파일 정독 완료나 완전한 독립 검증으로 표현하지 않는다. 통합 검증에서는 해당 문서의 결정과 제한도 직접 확인했다.

**중요한 구분:** 현재 실행 코드는 순수 codec 후보 선택뿐이다. 외부 P1을 실제 재생 장애나 현재 코드의 P1으로 채택하지 않았다. 다음 항목은 초기 설계의 빈 계약과 모호성을 구현 전에 해소하기 위한 제안이다.

## 검증된 설계 보완안

아래 line은 리뷰 대상 커밋 기준이다. 우선순위는 현재 구현 결함의 심각도가 아니라 설계 보완 순서다.

| ID | 우선순위 | 근거 | 확정할 내용과 최소 보완안 |
| --- | --- | --- | --- |
| D1 | P2 | `interfaces.md:80,88`, `architecture.md:108-112`, `device-status.md:23-25` | 저장한 pending과 다음 render demand가 사용할 revision의 관계가 없다. desired, 다음 재생용으로 준비한 revision, 현재 active의 역할을 정하고 SavePolicy만으로 다음 재생 대상을 바꾸는지 명시한다. fresh capability가 없거나 다음 재생에서 달라졌을 때의 결과도 정한다. |
| D2 | P2 | `interfaces.md:33-50,59,88`, `ui-design.md:149` | 재구성·resume·실패 복구 전이와 format revision 변경 규칙이 없다. stream lifecycle과 ApplyOperation을 분리하고, 적용 중 render demand 및 이전 설정 복구 실패의 처리와 유한 deadline을 명시한다. |
| D3 | P2 | `architecture.md:53`, `interfaces.md:90`, `ui-design.md:111-112` | 현재 PCM 경로와 codec 입력 형식의 변경 주체를 고정해야 한다. OS/driver PCM, 명시적 변환, encoder 입력의 소유자·허용 조합을 표로 정한다. 변환·재협상이 구현되기 전에는 실제 현재 PCM 경로와 다른 선택지를 비활성/읽기 전용으로 제한한다. |
| D4 | P2 | `interfaces.md:14,17,90`, `device-status.md:89-92` | codec별 parameter를 하나의 desired 묶음에서 fallback 후보에 어떻게 연결할지 미정이다. codec별 설정과 공통 PCM 제약을 구분하고, SBC용 설정이 없으면 어떤 검증된 기본/auto 정책을 쓸지 명시한다. 다른 codec의 bitrate·mode를 복사하지 않는다. |
| D5 | P3 | `interfaces.md:11-17,90`, `ui-design.md:109`, `codec.rs:102-111,129-156` | UI 단일 선택을 길이 1 preference 목록으로 매핑하는지 결정한다. 후속 협상 owner에서 원래 preference를 보존하고 시도별 제외 후보·실패 사유를 관리하는 계약을 추가한다. 현재 순수 API를 곧바로 재시도 버그로 판정하지 않는다. |
| D6 | P3 | `interfaces.md:106`, `device-status.md:56,72-78` | 완료 byte counter와 동일 관찰 시각, clock 단위, counter reset epoch를 함께 전달하도록 명시한다. epoch 변경과 불완전 window를 폐기하고, validity와 stream 상태가 stale 계산보다 우선하도록 정한다. |
| D7 | P3 | `device-status.md:78`, `ui-design.md:216` | stale의 “3초 이상”과 “3초 초과”가 불일치한다. 동일 경계로 통일하고 3초 경계 수용 기준을 맞춘다. |

D1~D4를 확정하기 전에는 서비스·driver·UI의 구현 계약이 완료되었다고 표시하지 않는다. 기존 M1의 binding·복구 실기 gate도 그대로 남는다.

## 외부 의견별 채택·기각

### 일반 리뷰

| 원문 항목 | 판정 | 근거 |
| --- | --- | --- |
| P2-1 Idle fresh capability 불가 | 강한 결론 기각, D1에서 보완 | `device-status.md:29`의 경쟁 AVDTP 금지는 WindowsDefault에 한정된다. `:25,37`은 ProjectDriver 소유 경로의 조회를 허용하므로 “항상 실패/조회 금지”는 성립하지 않는다. Idle query의 수명주기와 준비 결과를 더 명시할 필요는 있다. |
| P2-2 reconfigure/resume 누락 | D2 채택 | 사건 표와 상태도 사이에 경로·revision·복구 계약이 빠졌다. |
| P2-3 endpoint/encoder 형식 소유권 | D3 채택, 동작 실패는 미확인 | `ui-design.md:111-112`는 이미 PCM 교집합과 미구현 변환 제한을 둔다. 예시 옵션을 무조건 선택 가능하다고 읽는 것은 부정확하지만 변경 소유자를 명시할 필요가 있다. |
| P3-1 다음 후보와 순수 API 불일치 | 현재 코드 결함 기각, D5 보완 | `codec.rs:102-111`은 한 번의 후보 선정만 보장한다. 아직 협상 caller가 없으며, 가정한 preference 삭제 재시도는 현재 구현이 아니다. 순수 selection reason과 협상 시도별 오류 이력도 별도 책임이다. |
| P3-2 단일 선택과 목록 | D5 채택 | UI에서 서비스 preference로의 매핑 결정을 명시한다. |
| P3-3 Mono와 stereo 입력 충돌 | 결함 기각 | `device-status.md:89`, `ui-design.md:110,119`가 PCM 교집합과 별도 채널 검증을 요구한다. 지원되지 않는 Mono를 활성화하라는 계약은 없다. D3의 형식 표에 구체 예시를 둘 수 있다. |
| P3-4 counter 시각 | D6 채택 | 반환 시각과 계수 관찰 시각을 같은 것으로 처리하지 않도록 provider 계약을 고정한다. |
| P3-5 3초 경계 | D7 채택 | 실제 문안 불일치를 확인했다. |

### 적대적 리뷰

| 원문 항목 | 판정 | 근거 |
| --- | --- | --- |
| P1-1 pending이 다음 재생을 방해 | P1 기각, D1의 P2 설계 공백으로 채택 | 어떤 revision을 쓰는지 미정이라는 근거는 맞다. “마지막 저장본 사용으로 무음”은 원문도 추정으로 표시했으며 현재 서비스가 없어 실행 증거가 없다. UI 종료가 직접 stream을 멈춘다는 코드도 없다. |
| P1-2 endpoint 형식 소유자 없음 | P1 기각, D3으로 통합 | PCM 교집합·미구현 변환 제한이 있어 불가능한 옵션을 무조건 적용한다는 결론은 과하다. OS 형식 변경과 타 앱 영향은 실기 미확인이다. |
| P2-1 fallback parameter | D4 채택 | 후보 codec에 맞는 parameter 선택 정책은 추가로 결정해야 한다. |
| P2-2 reconfigure/rollback/apply | D2 채택 | rollback을 무조건 성공한다고 약속하지 말고 실패 결과까지 정의해야 한다. |
| P2-3 반복 후보·reason·UI 목록 | 현재 코드 결함 기각, D5로 통합 | 순수 후보 선택과 미래 협상 caller를 혼동하지 않는다. 다음 후보·오류 이력의 owner 계약은 필요하다. |
| P3-1 stale/counter | D6으로 부분 채택 | reset epoch·clock 결합은 보완한다. `device-status.md:78`은 중단 시 과거값 또는 해당 없음 표시를 허용하므로 모든 Idle을 stale로 만든다는 결론은 확정하지 않는다. |
| P3-2 시안 불일치 | 결함 기각, 구현 시 확인 | 합성 화면은 전체 장치 filter를 선택한 예시다. “입력 PCM 해상도” label 아래의 16 bit는 sample kind를 숨긴 별도 의미가 아니다. 고급 설정은 접혀 있고 dirty 값이 없는 화면에 변경 banner가 반드시 필요하지 않다. owner·미적용 상태와 fallback은 실제 해당 상태에서 검증한다. |

## 검증 결과와 한계

- 로컬 Windows, Rust 1.99.0에서 `pwsh -NoProfile -File scripts/check.ps1` Full 통과: 문서/YAML/TOML, diff, fmt, clippy, 정책 테스트 12건, doc test 1건, release build, rustdoc.
- 마지막 문안 보완 후 문서 검사와 diff 검사 재통과. 이번 변경에 Rust 실행 로직 변경은 없다.
- 시안은 Bluetooth 목록, 지원 codec, 현재 상태 6종, 설정 적용·저장만, codec별 선택지, font·spacing을 시각 확인했다. token 색상 대비는 계산으로 확인했다.
- UI·provider·encoder·driver가 없어 클릭, Narrator, DPI, 실제 codec·kbps, Bluetooth 재생·복구 시험은 미실행이다.
- M1의 driver binding, cancellation, PCM 경로·복구는 실기 증명이 필요하다. 이 정적 리뷰나 CI 통과가 이를 대신하지 않는다.
