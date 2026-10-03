# 공식 기술 근거와 확인 범위

확인일: 2026-10-03 (KST). 플랫폼·규격·권리자의 공식 자료를 사용합니다. 문서는 바뀔 수 있으며 source 반입 시에는 commit·artifact hash를 별도로 고정합니다. 아래 자료는 기술적 제약의 근거이며 제품 설계는 [요구사항](requirements.md)에서 도출합니다.

| 자료 | 이번 확인 범위 | 설계에 반영한 것 |
| --- | --- | --- |
| [Microsoft Bluetooth Classic Audio](https://learn.microsoft.com/en-us/windows-hardware/drivers/bluetooth/bluetooth-classic-audio) | 프로파일·기본 codec 지원 목록 | A2DP Source, SBC 우선, HFP/LE와 범위 분리 |
| [Bluetooth profile drivers](https://learn.microsoft.com/en-us/windows-hardware/drivers/bluetooth/bluetooth-profile-drivers-overview) | profile driver와 하위 L2CAP/SDP DDI | user mode 설정만으로 codec 추가를 가정하지 않음 |
| [L2CAP client connection](https://learn.microsoft.com/en-us/windows-hardware/drivers/bluetooth/creating-a-l2cap-client-connection-to-a-remote-device) | BRB open/close, MTU 협상 | kernel transport와 실제 협상 결과 사용 |
| [ACX overview](https://learn.microsoft.com/en-us/windows-hardware/drivers/audio/acx-audio-class-extensions-overview) | KMDF 기반, WaveRT, PortCls 공존 | endpoint 후보와 M1 검증 범위 |
| [windows-drivers-rs](https://github.com/microsoft/windows-drivers-rs) | 지원 구조·WDK 준비·초기 단계 안내 | Rust 우선, native bridge 가능성, 별도 driver workspace |
| [Microsoft driver signing](https://learn.microsoft.com/en-us/windows-hardware/drivers/install/kernel-mode-code-signing-policy--windows-vista-and-later-) | 개발·production 서명 경계 | 실기 시험과 공개 배포 분리 |
| [Bluetooth qualification](https://www.bluetooth.com/develop-with-bluetooth/qualify/) | 제품 자격 절차의 일반 안내 | 정확한 제품 구성별 적용·비용은 별도 확인 |
| [LDAC LICENSE](https://android.googlesource.com/platform/external/libldac/+/refs/heads/main/LICENSE) | Apache-2.0 | encoder 후보, 상표·기여 범위 밖 권리와 구분 |
| [aptX source](https://android.googlesource.com/platform/packages/modules/Bluetooth/+/refs/heads/main/system/embdrv/encoder_for_aptx/src/aptXbtenc.c) | Apache-2.0 header | 공개 encoder 후보 |
| [aptX HD source](https://android.googlesource.com/platform/packages/modules/Bluetooth/+/refs/heads/main/system/embdrv/encoder_for_aptxhd/src/aptXHDbtenc.c) | Apache-2.0 header | 독립 backend·실기 검증 |
| [FDK AAC NOTICE](https://android.googlesource.com/platform/external/aac/+/refs/heads/main/NOTICE) | CLI로 원문 확인, 특허 별도 안내 | AAC 기본 포함 보류 |
| [Via LA AAC](https://www.via-la.com/licensing-programs/aac/) | licensing program 페이지 | 적용 계약·가격을 제품 확정 후 재조회 |

## 검증을 미룬 항목

- Bluetooth AVDTP/A2DP normative revision과 해당 판의 세부 timing·packet 규칙: protocol 구현 전 원문과 적용 범위를 확정한다.
- SIG qualification의 이 소프트웨어 제품에 대한 적용·비용: 일반 source license만으로 결론 내리지 않는다.
- target WDK의 ACX Rust bindings, PnP binding, 기본 BthA2dp와 공존/대체: 실제 driver spike 필요.
- 실제 장비에서의 codec 지원과 낮은 지연: capability 조회·실기 측정 필요.
