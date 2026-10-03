# 적대적 리뷰 원문

> 아래는 외부 모델의 원문 의견이며 검증된 결함 목록이 아닙니다. [통합 검증 기록](2026-10-03-design-review.md)의 채택·기각과 모델 사용 기록을 함께 읽으세요.

- 대상 커밋: e577e875111fa63e07292ebf4ea39d4a9c1fc6e0

## Findings

### P1-1. pending으로만 저장된 설정을 render demand가 어떻게 쓰는지 정의되지 않음 (UI가 닫혀 있으면 재생 실패가 드러나지 않을 수 있음)
- **근거:** `docs/device-status.md:23-24`는 WindowsDefault 경로나 미연결 상태에서도 희망값을 pending으로 저장할 수 있게 합니다. `docs/interfaces.md:80`에서 SavePolicy는 remote가 확인되지 않은 값을 pending으로 둡니다. 반면 "다음 render demand용 설정 준비"는 ApplyPolicy에만 묶여 있습니다(`docs/interfaces.md:88`). `docs/architecture.md:108-112`는 render demand가 들어오면 `local × remote × policy`로 검증한다고만 쓰고, 그 policy가 어느 revision인지 밝히지 않습니다. 마지막 저장본인지 마지막으로 적용·검증된 값인지 알 수 없습니다. `crates/a2dp-core/src/codec.rs:49-55`의 기본값은 `FallbackPolicy::Disabled`이고, `docs/interfaces.md:131`의 NoCommonCodec에는 재시도가 없습니다.
- **실패 시나리오 (추정):** 사용자가 미연결 상태에서 LDAC 96 kHz를 pending으로 저장하고 창을 닫습니다. 나중에 Windows에서 연결하고 재생하면 서비스가 마지막 저장본을 씁니다. 장치가 LDAC를 지원하지 않거나 형식이 맞지 않으면 fallback이 꺼져 있으므로 NoCommonCodec이 됩니다. 이 기기에서는 소리가 나지 않고, UI가 없으니 이유도 보이지 않습니다. "닫기/종료가 재생을 막지 않는다"는 의도를 저장 경로가 우회해서 깨뜨리는 셈입니다.
- **최소 수정:** render demand가 쓰는 revision을 계약으로 정하세요. 마지막으로 검증된 revision과 미검증 pending을 분리하는 방식을 권장합니다. 그리고 pending을 충족할 수 없을 때의 결과를 정의하세요. 마지막 검증 설정으로 되돌리거나, 명시적 실패를 진단 snapshot에 남기는 정도면 됩니다.
- 이 값을 어디에 둘지는 P2-1과 같은 뿌리입니다.

### P1-2. OS endpoint PCM 형식과 encoder 형식 사이에 소유자가 없음
- **근거:** `docs/architecture.md:53`은 첫 형식을 48 kHz / 16-bit로 고정하고 숨은 resampling을 금지합니다. 그런데 `docs/ui-design.md:111-112`는 기기별로 샘플링 주파수(44.1/96 kHz 포함)와 입력 PCM 해상도(24-bit, 32-bit float)를 고르게 합니다. `docs/device-status.md:49-50`과 `:66`은 Windows mix format과 codec rate를 섞지 말라고만 하고, 누가 endpoint 형식을 결정하는지는 쓰지 않습니다. `docs/interfaces.md:90`은 PCM kind/bits를 desired에 저장하지만, 그 값이 driver가 광고하는 endpoint 형식을 바꾸는지, Windows 장치 형식 설정과 충돌하면 어느 쪽이 이기는지는 없습니다.
- **실패 시나리오:** resampling이 없으면 44.1 kHz나 24-bit를 선택하는 순간 endpoint 형식도 바뀌어야 합니다. 그러면 "Windows 설정·출력에 영향 없음"(`docs/ui-design.md:14`, `docs/architecture.md:43`)과 충돌합니다. 또 endpoint 형식이 바뀌면 같은 endpoint를 쓰는 다른 shared-mode 앱의 stream도 영향을 받습니다. 이 부분은 Windows 동작에 대한 추정이고 이 저장소 파일로는 확인하지 못했습니다. 이렇게 되면 `docs/ui-design.md:136`의 "오디오가 잠시 중단될 수 있음" 안내는 영향 범위를 실제보다 작게 설명합니다.
- **최소 수정:** "OS 형식 → (명시적 변환) → encoder 입력" 경계를 계약 표로 만드세요. 표에는 endpoint 형식 소유자, 변경 시 영향 범위, 지원하지 않는 조합의 UI 처리를 넣습니다. 변환 단계가 생기기 전까지는 선택지를 endpoint 형식으로 제한(readonly)하는 것이 가장 작은 수정입니다.

### P2-1. desired가 기기당 한 벌뿐이라 SBC fallback·다음 후보의 parameter가 정의되지 않음
- **근거:** `docs/interfaces.md:90`은 channel mode, bitrate, quality preset을 기기마다 하나씩만 저장합니다. 반면 `docs/interfaces.md:14,17`과 `docs/ui-design.md:117`은 SBC fallback과 다음 후보 시도를 허용합니다. `docs/device-status.md:89-92`는 이런 mode들이 codec마다 공유되지 않는다고 못박고 있습니다.
- **실패 시나리오:** 선호가 LDAC Dual Channel / 990 kbps이고 SBC fallback이 켜져 있다고 합시다. SBC로 떨어지면 저장된 parameter는 SBC에서 무효입니다. 이때 서비스가 auto로 바꿀지, 실패할지, 값을 몰래 고칠지 정해져 있지 않습니다. 몰래 바꾸면 `docs/interfaces.md:15`의 "몰래 다른 값을 저장하지 않음"과도 충돌합니다.
- **최소 수정:** desired를 codec별 parameter map으로 바꾸고, 해당 codec 항목이 없을 때의 규칙을 명시하세요. 예를 들어 "auto로 협상하고 reason=fallback을 보고"하는 식입니다.

### P2-2. 재생 중 reconfigure 경로와 실패 후 롤백, apply 작업 상태가 상태 모델에 없음
- **근거:**
  - `docs/interfaces.md:59`와 `docs/device-status.md:26`은 suspend → reconfigure 또는 stream 재시작을 요구합니다. 하지만 상태도(`docs/interfaces.md:33-50`)에는 Streaming에서 다시 설정 단계로 가는 전이가 없습니다. Suspended에서 나가는 길은 Discovering("reconnect with new generation")과 Stopping뿐입니다.
  - `docs/interfaces.md:86`은 적용 실패 시 desired를 보존한다고만 합니다. suspend 뒤 peer가 새 설정을 거절했을 때 이전 active 설정으로 재생을 복구하는지는 없습니다.
  - `docs/ui-design.md:149`는 lifecycle 상태인 `Configuring`을 "설정 적용 중 / 중복 적용 차단"으로 표시합니다. 그러나 상태도에서 Idle 중 ApplyPolicy는 Configuring에 들어가지 않으므로 이 차단은 작동하지 않습니다. 실제 보호 장치는 `docs/interfaces.md:88`의 Busy인데, 여기에는 apply 도중 들어온 render demand를 기다리게 할지, 이전 설정으로 진행할지가 없습니다. render demand는 Busy로 거절할 수도 없습니다.
- **실패 시나리오:** 재생 중 적용 → suspend → peer reject 순으로 진행되면 stream이 Suspended에 남거나 Stopping으로 갑니다. 사용자 입장에서는 "설정 적용"이 재생 중단으로 끝납니다.
- **최소 수정:**
  - 상태도에 `Streaming → Reconfiguring → Streaming | (rollback to prior active) | Stopping`을 추가하세요.
  - ApplyOperation 상태(진행/준비됨/적용됨/롤백됨/실패)를 lifecycle과 따로 정의하세요.
  - apply 도중 render demand가 오면 어떻게 처리하는지 명시하세요.

### P2-3. `select_codec` 단일 후보 API가 문서의 "다음 후보 시도"·reason 계약과 맞지 않음
- **근거:**
  - `docs/interfaces.md:17`은 상세 형식이 맞지 않거나 peer가 거절하면 다음 후보를 시도하라고 합니다. 하지만 `crates/a2dp-core/src/codec.rs:129-156`은 후보 하나만 반환하고, 이미 거절된 후보를 빼는 입력이 없습니다.
  - 가장 쉬운 우회는 거절된 codec을 `preferred`에서 빼는 것입니다. 그런데 `[LDAC]`에서 LDAC를 빼면 `codec.rs:135`에서 fallback 판정보다 먼저 `EmptyPreference`가 반환됩니다. 결과적으로 "LDAC가 identity 단계에서 불일치"하면 SBC로 fallback되지만, "LDAC가 형식 단계에서 실패"하면 설정 오류가 됩니다. 실패 단계에 따라 결과가 달라지는 것입니다. `local_encoders`에서 빼면 피할 수 있지만 문서에 그런 지침이 없습니다.
  - `SelectionReason`(`codec.rs:59-64`)은 Preferred/SbcFallback 두 가지뿐이라 `docs/interfaces.md:25`가 요구하는 reason 체인(peer reject, 미구현 등)을 담지 못합니다.
  - UI는 선호 codec을 단일 dropdown으로 고르는데(`docs/ui-design.md:109`, 시안도 동일), 정책은 순서 있는 목록을 전제합니다(`docs/interfaces.md:11`).
- **최소 수정:** 제외 집합을 입력으로 받는 반복 계약을 문서화하거나 API에 추가하세요. "거절된 후보는 local inventory에서 제외"처럼 정해도 됩니다. 시도별 reason을 보존할 구조를 interfaces에 정의하고, UI에 순서 목록을 둘지 단일 선택을 둘지 결정해야 합니다.
- 현재 테스트(`crates/a2dp-core/tests/codec_policy.rs`)는 단일 호출 불변식만 검증합니다.

### P3-1. 상태 계약의 작은 구멍
- `docs/device-status.md:78`의 3초 규칙을 그대로 적용하면, 갱신이 없는 Idle 상태의 필드가 `not_applicable`(`:63`)이 아니라 stale로 표시됩니다. 적용 대상을 active stream으로 한정해야 합니다.
- kbps를 어디서 관측하는지가 모호합니다. BRB 완료 기준(`docs/device-status.md:75`)인지, 서비스가 받는 ReadCounters IOCTL(`docs/interfaces.md:106`)인지 둘 다 해석할 수 있습니다. 또 counter의 "통계 reset generation"이 snapshot 식별자(`docs/device-status.md:56`)와 무효화 조건(`:77`)에 빠져 있습니다.

### P3-2. 시안과 명세의 표기 불일치 (배치와 문구 일관성만 본 것이며 실측 아님)
- 필터가 `전체 장치`로 보이는데, 명세의 기본값은 오디오 기기입니다(`docs/ui-design.md:56`).
- 해상도가 `16 bit`로 표시되는데, 명세는 `16 bit PCM`입니다(`docs/device-status.md:54`).
- SBC fallback opt-in(`docs/ui-design.md:117`), route owner 표시(`docs/device-status.md:12`), desired와 active 차이 표시가 시안에 없습니다.

**미확인·한계:**
- README.md, docs/sources.md, roadmap, validation, ADR 0001/0003, codec-and-distribution 문서는 키워드 검색으로만 훑었습니다.
- UI 코드, 실기 동작, endpoint 형식 변경이 다른 앱에 주는 영향은 확인하지 않았습니다.
- 테스트와 빌드는 실행하지 않았습니다.

## Ship / No-ship
**지금 상태로는 계약 확정에 동의하지 않습니다.** 세부 문서 수준은 높습니다. 하지만 핵심 요구인 "UI가 닫혀도 재생에 영향 없음"과 "OS 형식과 encoder 형식의 경계"를 저장 경로와 형식 선택이 우회할 수 있고, 그 우회를 막는 계약이 없습니다.

**견고한 부분:**
- 상태별 관찰 주체를 분리했습니다(paired/link/endpoint/route/stream).
- validity 5종을 두고, 0이나 기본 codec으로 값을 채우지 않게 금지했습니다.
- request, generation, revision 기반으로 늦은 결과를 거부합니다.
- 코드와 테스트 모두 fallback을 몰래 하지 않습니다.
- cancel/buffer 소유권 순서가 정해져 있습니다(`docs/interfaces.md:112,119`).

**취약한 부분:**
- pending 설정이 언제 소비되는지 정해져 있지 않습니다.
- endpoint 형식에 소유자가 없습니다.
- desired가 기기당 한 벌이고, reconfigure와 롤백이 모델에 없습니다.

**우선 개선 3가지:**
1. render demand가 쓰는 revision과 그 값을 충족할 수 없을 때의 degraded 결과를 정의합니다.
2. OS 형식 → 변환 → encoder 경계를 소유자와 영향 범위까지 포함해 계약 표로 만들고, 그 전까지 형식 선택지는 readonly로 둡니다.
3. desired를 codec별로 만들고, Reconfiguring 전이·롤백·ApplyOperation 상태와 후보 제외 반복 계약을 추가합니다.
