# 설계 결정 기록

| ADR | 상태 | 결정 |
| --- | --- | --- |
| [0001](0001-rust-and-windows-boundary.md) | 수용, driver 세부 방식은 검증 전 | Rust 중심, 작은 Windows/native 경계, user mode codec 우선 |
| [0002](0002-codec-availability.md) | 수용 | 후보 정책·실제 backend·원격 지원·배포 조건 분리 |
| [0003](0003-hardware-gates.md) | 수용 | SBC 실기·복구를 먼저 증명한 뒤 확장 |

새 정보가 기존 결정을 바꾸면 변경 이유와 대체 ADR을 남깁니다. 과거 결정 내용을 지워 현재 검증 결과처럼 보이게 만들지 않습니다.
