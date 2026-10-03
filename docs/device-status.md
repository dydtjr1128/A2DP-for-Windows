# 장치 상태·코덱·측정값 계약

상태: 설계 계약, provider와 실기 검증 전. Windows 연결, 앱의 오디오 경로, 지원 능력, 현재 전송값을 각각 관찰하고 출처와 유효성을 함께 전달합니다. UI 표현은 [UI 설계](ui-design.md)를 따릅니다.

## 1. 서로 다른 상태의 분리

| 상태 | 관찰 주체·후보 API | 의미 | 의미하지 않는 것 |
| --- | --- | --- | --- |
| paired | Windows 장치 열거·pairing 정보 | Windows에 등록된 장치 | 연결됨·A2DP 지원 |
| bluetooth_link | Windows BluetoothDevice.ConnectionStatus | Windows가 보고한 장치 연결 상태 | 현재 codec·오디오 재생 |
| endpoint_state | Windows IMMDevice::GetState | audio endpoint의 활성·비활성·분리 상태 | 현재 stream에서 쓰는 format·codec |
| route_owner | 설치·PnP·서비스가 확인한 function binding | WindowsDefault / ProjectDriver / Unknown | UI 선택만으로 소유권 이전 |
| stream_state | 해당 owner의 실제 session provider | Idle / Configuring / Streaming 등 | 헤드폰에서 실제 소리를 들었다는 증거 |

Windows 연결을 bool 하나로 현재 오디오 전체 상태에 대입하지 않습니다. 각 관찰이 실패하면 해당 필드만 unknown으로 표시합니다. 장치 선택·설정 열기는 pairing, 연결, driver binding, 기본 출력 장치를 변경하지 않습니다.

공식 근거: [Bluetooth 연결 상태](https://learn.microsoft.com/en-us/uwp/api/windows.devices.bluetooth.bluetoothdevice.connectionstatus), [endpoint 상태](https://learn.microsoft.com/en-us/windows/win32/api/mmdeviceapi/nf-mmdeviceapi-immdevice-getstate).

## 2. 경로별 권한과 가용성

| 현재 경로 | 조회 | 설정 저장 | 현재 출력에 적용 |
| --- | --- | --- | --- |
| WindowsDefault | 확인 가능한 Windows 장치·endpoint 상태만. codec telemetry는 검증한 provider가 없으면 unavailable | local schema·backend 검증을 만족하는 희망값을 pending으로 저장 가능 | 불가. 이 앱이 기본 Windows codec을 임의 변경할 수 있다고 가정하지 않음 |
| ProjectDriver, 장치 미연결 | 현재 link 상태, 명시적으로 표시한 과거 capability | local 검증 후 pending 저장 | 불가. Windows에서 연결한 뒤 새 검증 필요 |
| ProjectDriver, link 연결·stream 대기 | 해당 driver가 허용하는 fresh capability, 검증된 설정 | local·format validation | 다음 stream의 설정으로 준비 가능. 재생이나 Bluetooth 연결을 강제로 시작하지 않음 |
| ProjectDriver, Streaming | 동일 session/format revision의 실제 관찰값 | revision 검사 | 지원하는 reconfigure 또는 stream 재시작. 중단 영향 사전 안내 |
| Unknown / 권한 실패 / version 불일치 | 확인 가능한 항목과 실패 이유 | 저장 서비스·schema를 확인하지 못하면 불가 | 불가, 원인에 맞는 안내 |

WindowsDefault 경로에서 codec 목록을 얻기 위해 별도 AVDTP 채널을 경쟁적으로 열거나, UI 조회만으로 driver를 교체하지 않습니다. 적용을 위해 경로 전환이 필요하면 검증된 설치·복구 절차의 독립 작업으로 분리합니다. 일반 설정 적용에 관리자 설치나 binding 변경을 숨기지 않습니다.

## 3. 지원 코덱 목록의 세 가지 의미

- **장치 지원:** 현재 peer의 AVDTP capability로 확인한 codec identity와 parameter 조합.
- **앱 구현:** 이 빌드에 탑재되어 초기화·설정이 가능한 encoder 목록. 현재 저장소에는 encoder가 없다.
- **사용 가능:** fresh peer capability, 활성 local backend, 현재 route와 PCM 경로, 배포 정책을 모두 만족하는 후보.

SDP는 서비스를 발견하는 데 사용하며 codec 목록의 근거로 쓰지 않습니다. 실제 capability는 profile 소유권이 확인된 경로에서 AVDTP endpoint discovery/GetCapabilities를 통해 얻습니다. 목록 조회 실패와 지원 codec이 없는 결과를 구분합니다. 캐시에는 관찰 시각·장치 identity·provider revision을 저장하고 **이전 확인값**으로 표시합니다. 캐시는 현재 적용 권한이나 live capability를 대신하지 않습니다.

UI는 기본 codec 이름을 보기 좋게 채우지 않습니다. unknown이면 **지원 코덱 확인 불가**, 조회 중이면 **확인 중**으로 표시합니다. codec 이름 옆에 사용 가능·앱 미구현·장치 미지원·현재 경로에서 설정 불가를 각각 설명합니다.

## 4. 현재 오디오 상태 필드

아래 필드는 `CurrentAudioSnapshot`이라는 논리 계약입니다. 직렬화 wire schema는 구현 단계에서 고정합니다.

| UI 항목 | 값과 단위 | authoritative source | 표시 제한 |
| --- | --- | --- | --- |
| 코덱 유형 | codec identity | 현재 session의 accept된 codec configuration | 선호값·기본 OS 지원 목록으로 추정하지 않음 |
| 채널 모드 | codec별 negotiated channel mode | codec configuration와 encoder active state | PCM 2 channels만 보고 Joint Stereo라고 표시하지 않음 |
| 샘플링 주파수 | Hz, UI는 kHz | codec configuration의 active sampling frequency | Windows mix format과 codec rate를 같은 필드로 혼합하지 않음 |
| 입력 PCM 해상도 | sample kind, valid bits, container bits | encoder로 전달되는 검증된 PCM format | 압축 codec의 무손실 해상도·원음 정밀도를 뜻하지 않음 |
| 전송률 | decimal kbps | 성공 완료된 encoded audio payload byte counter의 시간 차분 | 설정 목표값·Bluetooth PHY 속도와 구분 |
| 품질 설정 | active preset/mode, 필요 시 현재 적응 level | encoder active parameters | 청감 품질 점수나 헤드폰 성능 평가가 아님 |

`PCM 해상도`는 유효 bit와 container를 구분합니다. 예를 들어 유효 24 bit가 32-bit container에 있으면 기본 표시는 **24 bit PCM**, 상세는 **32-bit container**입니다. float은 **32-bit float**처럼 sample kind를 함께 표시합니다. SBC와 같은 압축 bitstream에 보편적인 16/24-bit 해상도가 있다고 표현하지 않습니다. [WAVEFORMATEXTENSIBLE](https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatextensible)

한 snapshot에는 device ID, route owner, session generation, format revision, provider, 관찰 시각과 각 필드의 validity를 포함합니다. 서로 다른 장치·generation·format revision의 codec·전송률·format 값을 합쳐 한 줄에 표시하지 않습니다.

| 필드 유효성 | UI 표현 |
| --- | --- |
| known | 실제 값과 단위 |
| querying | 확인 중 |
| unavailable | 확인 불가 + 이유 |
| not_applicable | 해당 없음 |
| stale | 마지막 확인값 + 경과 시간, 현재 값과 다른 표현 |

값이 없다는 이유로 0, SBC, stereo, 48 kHz를 채우지 않습니다. WindowsDefault에서 PCM endpoint 정보만 확인할 수 있으면 **Windows PCM 형식**이라는 별도 상세 항목으로 표시합니다.

## 5. 전송률 계산과 노후화

앱의 encoded audio payload 전송률 제안식은 다음과 같습니다.

`kbps = 8 × (completed_payload_bytes_now - completed_payload_bytes_before) / elapsed_seconds / 1000`

- codec frame의 payload byte를 누적한다. RTP/AVDTP/L2CAP header, 재전송과 RF overhead는 제외한다.
- BRB 성공 완료는 앱이 하위 스택에 전송 완료로 관찰한 기준이다. 헤드폰 수신·무선 ACK를 증명하지 않는다. UI 설명은 **오디오 데이터, 앱 측 완료 기준**이다.
- 최소 1초의 유효 window가 확보된 뒤 기본 1초 간격으로 갱신한다. 실제 elapsed monotonic time을 사용하고 polling 간격을 고정값으로 대입하지 않는다.
- 첫 window, counter 감소/reset, generation/format revision 변경, 0 이하 elapsed에서는 결과를 만들지 않는다. codec 변경 시 이전 codec의 window를 이어 쓰지 않는다.
- snapshot 관찰 후 3초 이상 갱신되지 않으면 stale로 표시한다. stream이 중단되면 수치를 과거값으로 남기거나 해당 없음으로 표시한다.
- 동일 active stream의 완전한 유효 window에서 실제 완료 byte가 0인 경우만 **0 kbps**를 표시할 수 있다. 측정 불가를 0으로 표시하지 않는다.

설정 화면의 **목표 비트레이트**와 현재 상태의 **관측 전송률**을 별도 필드로 둡니다. 고정 목표와 관측 평균이 정확히 같다고 약속하지 않습니다.

## 6. 채널 수·스테레오 모드 선택

PCM 채널 수와 codec의 channel coding mode는 별도 값입니다. SBC의 Joint Stereo·Stereo·Dual Channel은 모두 2-channel 입력을 다룰 수 있지만 같은 encoding mode가 아닙니다. UI의 **스테레오 모드**는 codec-specific coding mode를 설정합니다.

| codec | 설정 후보 | 적용 조건 |
| --- | --- | --- |
| SBC | 자동, Mono, Dual Channel, Stereo, Joint Stereo | local encoder·peer capability·PCM 경로의 교집합. 모드를 바꾸면 bitpool·frame 길이·bitrate도 재검증 |
| LDAC | 자동, Mono, Dual Channel, Stereo 중 구현된 조합 | 공식 encoder API와 peer에 있는 모드만. Joint Stereo를 SBC에서 그대로 가져오지 않음 |
| AAC | 구현과 협상에 맞는 1/2-channel 선택 | SBC coding mode 옵션을 노출하지 않음 |
| aptX 계열 | 해당 backend가 실제 제공하는 mode | codec 이름이 유사하다는 이유로 mode를 공유하지 않음 |

단일 모드만 가능하면 읽기 전용으로 표시하고 이유를 설명합니다. 자동은 협상 정책의 희망값이며 현재 상태에는 실제 선택된 mode를 보여줍니다. frontend와 서비스가 동일한 validation을 수행하며 숨은 field를 보내 지원하지 않는 모드를 강제할 수 없어야 합니다.

공식 정의 확인: [SBC constants](https://android.googlesource.com/platform/packages/modules/Bluetooth/+/refs/heads/main/system/stack/include/a2dp_sbc_constants.h), [LDAC encoder API](https://android.googlesource.com/platform/external/libldac/+/refs/heads/main/inc/ldacBT.h). 이 자료의 구현 정책을 제품 설계로 복제하지 않으며 field 의미만 확인합니다.

## 7. 품질과 설정 선택지

품질은 encoder 설정입니다. backend가 정의한 안정성·균형·음질 preset, 고정 품질 또는 검증된 적응 mode만 노출합니다. 프리셋을 수동 변경하면 **사용자 설정**으로 표시합니다. 자동 품질이면 희망 mode와 실제 현재 level을 각각 표현할 수 있어야 합니다.

LDAC의 공식 품질별 bitrate 값은 sampling frequency 계열에 따라 다릅니다. 990/660/330 kbps를 모든 sample rate에 고정 적용하지 않습니다. AAC VBR, SBC bitpool, aptX의 고정 제약도 공통 slider 하나로 강제하지 않습니다. UI 옵션은 backend가 전달한 enum 또는 min/max/step에 한정합니다.

현재 workspace에는 codec backend·telemetry provider가 없으므로 production UI에서 위 상태를 실제 값으로 표시할 수 없습니다. 설계 이미지는 의미와 배치를 설명하는 합성 데이터입니다.
