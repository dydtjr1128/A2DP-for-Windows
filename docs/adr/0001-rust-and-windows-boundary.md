# ADR 0001: Rust 중심과 작은 Windows 경계

- 날짜: 2026-10-03
- 상태: 수용. ACX/PortCls 및 profile binding은 M1 검증 전.

## 배경

사용자는 Rust 중심 구현을 희망합니다. 핵심 위험은 일반 앱 코드가 아니라 Windows 오디오 endpoint, Bluetooth profile 소유권, 비동기 driver 수명주기입니다. Rust driver 도구의 존재만으로 이 경로가 증명되지는 않습니다.

## 결정

OS 독립 정책과 protocol을 Rust로 작성합니다. 서비스와 codec worker는 사용자 모드에 둡니다. kernel/native wrapper는 작은 경계로 분리하고 driver crate는 별도 WDK workspace에 둡니다. ACX를 우선 조사하되 bindings·PnP·PCM 흐름 증거가 부족하면 PortCls/WaveRT 또는 C/C++ bridge를 비교합니다.

## 대안과 결과

| 대안 | 고려 결과 |
| --- | --- |
| 전체 C/C++ | WDK sample 재사용에는 유리하나 사용자 방향과 pure logic 검증에 불리 |
| 모든 구성 Rust kernel | codec runtime·할당·FFI 위험과 도구 제약을 한번에 해결해야 하므로 초기 제외 |
| user mode 앱만 사용 | 기존 A2DP codec 교체 경로의 근거가 없어 목표 달성 수단으로 선택하지 않음 |
| Rust + 최소 native 경계 | 테스트 가능한 영역을 확보하고 미확인 Windows 경계를 별도 증명 가능 |

user mode data path는 scheduling·copy 비용이 생깁니다. [검증 계획](../validation.md)의 실측으로 수용 여부를 판단하며 미측정 상태에서 저지연을 보장하지 않습니다.

## 재검토 조건

profile binding 불가, PCM 획득 불가, 실측 지연·안정성 기준 미달, 필수 WDK API 미지원이면 driver 모델을 재검토합니다. 상세 경계는 [아키텍처](../architecture.md)를 따릅니다.
