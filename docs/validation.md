# 검증과 실기 수용 기준

## 증거 수준

| 수준 | 확인하는 것 | 증명하지 못하는 것 |
| --- | --- | --- |
| 문서·정적 검사 | 링크·YAML·설정·컴파일 정합성 | Bluetooth 연결·재생 |
| OS 독립 단위 테스트 | 정책·parser·계산·상태 경계 | Windows DDI·driver load·장비 호환 |
| user mode codec 시험 | known PCM의 encode 결과·실패·메모리 | peer 협상·무선 전송 |
| driver build/load 시험 | ABI·서명·PnP 초기 동작 | 장시간 재생·모든 전원 상태 |
| 실기 통합 | 선정 조합의 실제 기능 | 미시험 조합·미측정 지연 |
| 배포 검증 | 서명·설치·제거·고지·지원 matrix | 자동으로 모든 codec 권리 문제 해결 |

보고서는 통과·실패·미실행·건너뜀을 구분합니다. unit test의 모의 capability는 하드웨어 측정 데이터가 아닙니다.

## 자동 검사

문서 검사는 공식 자료·본 저장소에 해당하는 외부 링크의 도메인과 경로를 확인합니다. URL 응답과 문안의 의미는 자동 판정하지 않으므로 문서·코드·이미지·PR에 대한 수동 검토도 수행합니다.

M0는 codec 선호 순서, strict 실패, SBC fallback opt-in, empty input, local/remote 불일치와 빈 backend inventory를 검증합니다. 일반 library는 `no_std`와 `unsafe` 금지를 유지합니다. Windows와 Linux에서 같은 pure logic을 검사하되 Linux 통과를 Windows driver 검증으로 표현하지 않습니다.

후속 protocol/backend에서는 다음 경계를 추가합니다.

- 잘린 AVDTP/codec capability, 잘못된 길이·label·reserved bit·unknown vendor, 최대 길이
- frame 크기·MTU 최소/정확 경계/초과, sequence wrap, timestamp·길이 arithmetic overflow
- 중복·늦은 completion, stop와 removal 경합, timeout 뒤 cancellation completion
- codec FFI 실패·손상 입력·작은 output buffer, 재설정·flush·drop의 ownership
- IPC 부분 읽기·oversize·version mismatch·unauthorized client·stale generation
- 설정 임시 파일 쓰기 실패·손상 JSON·미지원 schema·atomic replace 실패

parser fuzz는 user mode에서 먼저 수행하고 crash sample은 합성 데이터만 저장합니다. 실제 사용자 오디오나 장치 식별자를 regression fixture로 넣지 않습니다.

## 실기 환경 표

| 항목 | 필수 기록 |
| --- | --- |
| 코드 | commit SHA, dirty 여부, build profile, driver·codec source revision |
| 도구 | Rust, WDK, SDK, Visual Studio, verifier 설정 |
| Windows | edition/build/architecture, 전원 정책, 테스트 서명·Secure Boot 상태 |
| radio | 모델·버스·driver version, 내부 시험용 비공개 instance ID |
| peer | 모델·firmware·공표 codec, 실제 읽은 capability |
| 환경 | 거리, 장애물, 전파 혼잡, 전원·USB hub 조건 |
| 입력 | 합성 tone 또는 사용 권한이 있는 오디오, PCM format·duration |
| 결과 | active codec·설정, duration, drops/underruns, disconnect reason, 복구 결과 |

공개 보고에서는 device address, serial, user path, 녹음·dump를 제외합니다.

## UI 검증

[UI 상세 설계](ui-design.md)의 UX-01~12를 구현 수용 기준으로 사용합니다. 목업 이미지는 시각 검토 자료이며 클릭·키보드·Narrator·고대비·DPI·service 연동 검증을 대체하지 않습니다. 구현 후에는 준비 실패, 상태 조회 실패, 설정 저장/적용 분리, stale 응답, 장치 전환, 연결 종료와 복구 실패를 실제 UI에서 확인합니다.

## 초기 실기 목표

아래는 **시험 계획의 제안값**이며 아직 측정한 결과가 없습니다.

| 단계 | 시나리오 | 목표와 판단 |
| --- | --- | --- |
| M1 | SBC 합성 신호 10분 | 실제 청취·전송 증거, BSOD·무한 queue 없음, 제거 후 기본 재생 |
| M2 | Windows 앱 재생 30분 | underrun/drop 기록, 지속 재생 확인, 오류 시 유한 실패·복구 |
| M2 | 재연결 20회 | old generation audio·핸들 누수 없음, 시도별 결과 기록 |
| M2 | sleep/resume 10회 | 매회 새 capability·MTU, 복구 또는 구체적 실패 이유 |
| M4 | soak 8시간 | memory/handle 기준선 대비 지속 증가 없음, 오류·지연 분포 기록 |
| M4 | 재연결 100회 / 절전 50회 | 성공률·실패 원인 기록, 회복 불가 시 release gate 실패 |
| M4 | 설치·갱신·제거·중도 실패 | 다른 Bluetooth 장치 보존, stock audio 복구, orphan service/package 없음 |

성공률은 분모·시험 횟수·실패를 함께 표기합니다. 한 번 소리가 난 결과로 전체 행을 통과 처리하지 않습니다.

## 성능 측정

- encode 시간 p50/p95/p99, PCM·media queue 깊이, transport completion 시간, drop·underrun을 수집한다.
- 같은 장비·음원·radio 환경에서 기본 드라이버와 비교한다. 호스트 처리량·queue 시간·음향 end-to-end를 분리한다.
- end-to-end 지연은 loopback/외부 계측의 방법·정확도·음향 경로를 함께 남긴다. 소프트웨어 timestamp만으로 헤드폰 내부 지연을 알 수 있다고 주장하지 않는다.
- profiler·ETW 수집 오버헤드와 sampling 주기를 기록한다. 드라이버 callback에 문자열 로그를 무제한 출력하지 않는다.

## 장애 주입과 복구

장치 제거, radio disable, peer power-off, service 강제 종료, encoder error, malformed packet, queue exhaustion, resume 중 제거를 단계별 주입합니다. 각 경우 admission 정지, outstanding request 회수, 자원 해제, 사용자 상태를 확인합니다.

Driver Verifier와 테스트 서명은 전용 시험 장치에서 계획·복구 경로를 준비한 뒤 적용합니다. VM만의 결과로 실제 radio·전원 호환성을 주장하지 않습니다. 현재 작업에서는 호스트의 부팅 설정이나 driver binding을 변경하지 않습니다.

## 결과 기록 양식

```text
시험 ID / 요구사항 ID:
날짜(KST), 담당자:
commit / source revisions / toolchain:
장비·OS·전원·radio 조건:
입력과 단계 / 기대 결과:
실제 결과 (통과·실패·미실행·건너뜀):
active codec / metrics / log 위치:
실패 원인 (확정·추정·미확인):
cleanup / stock audio 복구 결과:
남은 범위 / 다음 검증:
```

실기 결과는 향후 `docs/test-results/`에 비식별화하여 기록합니다. 아직 실행하지 않은 시험의 결과 파일을 통과 예시로 생성하지 않습니다.
