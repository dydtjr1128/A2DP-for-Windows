# 코덱과 배포 조건

기준일: 2026-10-03. 아래는 **검토 후보의 소스 조건**과 개발 순서이며, 이 프로젝트의 코덱 지원 완료·권리 승인 목록이 아닙니다. 현재 외부 코덱 소스·binary·SDK는 포함하지 않습니다.

## 후보와 순서

| 코덱 | 후보 구현·관찰한 조건 | 개인 개발 | 공개·상업 배포 전 확인 | 도입 순서 |
| --- | --- | --- | --- | --- |
| SBC | BlueZ libsbc, LGPL-2.1-or-later | source 조건을 지키는 실험 후보 | 링크 방식, 수정 소스·고지·재링크 조건; 대안 구현 비교 | 첫 실기 |
| LDAC | AOSP libldac encoder, Apache-2.0 | 공개 encoder 시험 후보 | 원본·수정 고지, 특허 grant 범위, 상표·인증 별도 확인 | SBC 이후 |
| aptX | AOSP encoder_for_aptx, Apache-2.0 표기 | 공개 encoder 시험 후보 | 정확한 반입 파일 전체 조건과 상표·제3자 권리 | LDAC 이후 개별 |
| aptX HD | AOSP encoder_for_aptxhd, Apache-2.0 표기 | 공개 encoder 시험 후보 | 일반 aptX와 별도 backend·format·실기 검증 | aptX와 별도 |
| AAC | AOSP FDK AAC, 독자 라이선스 | 조건을 검토한 개발 시험 후보 | 코드 사용 조건 외 특허 계약 적용 범위 확인 필요 | 조건부 보류 |
| aptX Low Latency | PipeWire 등의 연동 구현 참고 | 프로토콜·상호운용 연구 후보 | LL 전용 협상·구현·특허·상표·인증 근거 미확정 | 조건부 보류 |

근거: [SBC](https://kernel.googlesource.com/pub/scm/bluetooth/sbc/+/refs/heads/master/sbc/sbc.c), [LDAC](https://android.googlesource.com/platform/external/libldac/+/refs/heads/main/LICENSE), [aptX](https://android.googlesource.com/platform/packages/modules/Bluetooth/+/refs/heads/main/system/embdrv/encoder_for_aptx/src/aptXbtenc.c), [aptX HD](https://android.googlesource.com/platform/packages/modules/Bluetooth/+/refs/heads/main/system/embdrv/encoder_for_aptxhd/src/aptXHDbtenc.c), [FDK AAC](https://android.googlesource.com/platform/external/aac/+/refs/heads/main/NOTICE), [AAC program](https://www.via-la.com/licensing-programs/aac/), [PipeWire](https://github.com/PipeWire/pipewire/blob/master/spa/plugins/bluez5/a2dp-codec-aptx.c).

개인용이라는 이유만으로 모든 권리 조건이 없어지지는 않습니다. Apache-2.0의 소스 사용 허용을 모든 제3자 특허·제품 인증·로고 사용 허가로 확대하지 않습니다. FDK AAC의 source 사용 조건에는 필요한 특허권의 별도 확인이 명시되어 있습니다.

## Backend 도입 체크포인트

1. upstream URL, exact commit, 반입 파일, copyright·LICENSE·NOTICE를 기록한다.
2. 사용자 모드에서 standalone encode, 메모리 소유권, 출력 frame 크기, 오류 처리를 검증한다.
3. 알려진 입력과 golden vector 또는 독립 decoder로 bitstream을 검사한다.
4. capability parsing, 형식 교집합, codec-specific media payload를 별도 검증한다.
5. 선정 헤드폰에서 협상·재생·bitrate 변경·재연결을 확인한다.
6. 실제 backend inventory에 추가하고 UI에서 구현·원격 지원·정책 불가 이유를 구분한다.
7. 공개 artifact에 포함하기 전에 배포 방식별 조건·고지·소스 제공·서명을 확인한다.

기술 단계 통과와 배포 조건 확인은 다른 기록입니다. feature flag 하나로 양쪽 완료를 대신하지 않습니다. AAC/LL은 기본 빌드에 몰래 포함하지 않습니다.

## 개인 실험과 배포 경계

| 항목 | 개발 시험 | 공개·판매 배포 |
| --- | --- | --- |
| 장비 | 보유 PC·어댑터·헤드폰을 선정 | 지원 모델별 회귀·복구 검증 기록 |
| Driver 서명 | 전용 시험 환경의 테스트 서명 경로 검증 | 목표 Windows 정책에 맞는 Microsoft 제출·서명·검사 |
| Bluetooth 자격 | 구현 범위와 규격 의존성 조사 | 제품 구성 기준으로 SIG 적용 범위 확인 |
| 코덱 권리 | source/SDK 조건 확인 | 배포 형식·지역·상표·특허 등 적용 조건 확인 |
| 프로젝트 라이선스 | 아직 미결정 | 권리자가 정하고 소스·artifact 고지와 일치 |
| 지원·복구 | 선정 장비의 수동 복구 | 업그레이드·제거·실패 복구와 지원 정책 |

공개 GitHub 저장소에 코드가 있다는 사실만으로 배포 라이선스가 선택된 것은 아닙니다. 현재 Cargo package를 추가해도 `publish = false`로 유지합니다.

과거 대화의 인증서·등록비·환율은 예산 시나리오였으므로 이 설계에 확정 비용으로 복제하지 않습니다. 구매·계약 시점에 해당 공급자와 제품 범위를 다시 확인합니다. 드라이버 서명의 기준 자료는 [Microsoft 정책](https://learn.microsoft.com/en-us/windows-hardware/drivers/install/kernel-mode-code-signing-policy--windows-vista-and-later-)이며, SIG 확인은 [공식 qualification 안내](https://www.bluetooth.com/develop-with-bluetooth/qualify/)를 출발점으로 합니다.
