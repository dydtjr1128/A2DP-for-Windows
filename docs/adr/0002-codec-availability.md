# ADR 0002: 코덱 목표와 실제 사용 가능 상태 분리

- 날짜: 2026-10-03
- 상태: 수용

## 배경

원하는 코덱은 6종이지만 source 조건, encoder 구현, 원격 헤드폰 능력과 상세 형식이 다릅니다. 이름 목록만 보고 사용할 수 있다고 표시하면 연결 실패와 배포 오해를 만듭니다.

## 결정

SBC부터 실제 경로를 검증합니다. 순수 core는 codec 후보 정책만 담당합니다. 실제 local encoder inventory는 기본적으로 비우며 backend와 검증이 준비될 때 확장합니다. 선호 목록과 local/remote 능력의 교집합만 후보로 반환하고 SBC fallback은 명시적 허용이 있어야 합니다.

AAC와 aptX LL은 기본 포함을 보류합니다. 개인 시험과 공개·상업 배포의 검토는 별도 기록으로 남깁니다. `desired`, `proposed`, `active`를 구분하고 협상·Start가 확인되어야 active로 표시합니다.

## 대안과 결과

모든 코덱을 처음부터 dependency로 넣으면 미사용 바이너리·권리 조건·빌드 위험까지 늘어납니다. SBC를 항상 암묵적으로 선택하면 사용자 의도와 실제 출력이 달라집니다. 현재 선택은 단계별 통합 비용을 늘리지만 실패 이유와 기능 범위를 확인할 수 있습니다.

구체적인 backend와 조건은 [코덱 문서](../codec-and-distribution.md), 선택 계약은 [인터페이스](../interfaces.md)를 따릅니다. codec의 sample rate·bitrate까지 맞는지는 후보 선택 이후에 확인해야 합니다.
