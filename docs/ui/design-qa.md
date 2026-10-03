# 설정 UI 시안 검증

- 날짜: 2026-10-03 (KST)
- final result: passed
- 범위: HTML 시안의 시각 크기와 모의 상호작용. Windows 앱·실기 동작 수용 시험이 아님
- source visual truth: [수정 배치 참고 이미지](layout-reference.png), [UI의 확정 치수](../ui-design.md)
- implementation: [HTML](device-settings.html), [기본 화면](device-settings.png), [편집 중](qa/editing.png), [좁은 창](qa/narrow.png)

## 비교 조건과 의도한 변경

ImageGen 참고 이미지 원본은 1,513×1,039 px다. 1,280 px 폭으로 비율을 유지하면 높이는 약 879 px로, 기본 캡처 1,280×880 px와 같은 화면 비율이다. 생성 이미지에는 CSS 크기나 물리 DPI 정보가 없으므로 원본의 글자 픽셀 크기를 제품 기준으로 사용하지 않는다.

같은 어두운 테마, 오디오 기기 필터, 책상 헤드폰, SBC·Joint Stereo·48 kHz·16 bit PCM·균형의 편집 전 상태를 비교했다. source와 구현 캡처를 같은 비교 입력에서 열고 배치·문구·control을 확인했다. 원본보다 작은 14/20 CSS px 글꼴, 224 px 목록, 32 px control은 사용자가 요청한 축소·정돈을 위한 명시적 변경이다. 이미지의 근사 치수에 대한 1:1 복제 결과로 보고하지 않는다.

별도 확대 이미지 없이 1배율 전체 캡처에서 control label·값·버튼을 읽을 수 있었다. 해당 영역의 높이·너비·font·선택 상태는 실제 DOM 치수와 함께 검사했다. 플랫폼 창의 최소화·닫기 장식은 HTML 안에 중복 배치하지 않았다.

## Findings와 수정 이력

- 1차 [P2] 기기 filter 외곽만 34 px
  - 근거: 안쪽 button의 32 px 최소 높이에 wrapper border 2 px가 더해졌다.
  - 수정: 바깥 32 px, 안쪽 30 px로 조정.
  - 재검증: search, filter, select 5개, 품질 segmented의 외곽이 모두 32 px. 수정 후 기본 화면과 같은 source를 다시 비교했다.
- 최종: 남은 P0/P1/P2 없음. 미변경 상태에서 버튼이 비활성인 것은 의도한 동작이며 편집 상태 캡처에서 동일 크기의 활성 버튼을 확인했다.

## 필수 시각 검토

| 영역 | 확인 결과 |
| --- | --- |
| 글꼴·위계 | 본문/입력/상태값/버튼 14 px, 제목 20 px, 보조 12 px. 실제 플랫폼 font는 영문 Segoe UI Variable Text, 제목 Segoe UI Variable Display와 한글 Malgun Gothic으로 확인. 제목 semibold와 일반 label을 구분 |
| 간격·정렬 | 224 px 목록, 28 px main padding, 48 px form 행, 28 px 열 간격. control 외곽 32 px·radius 4 px·border 1 px. 하단 세 버튼은 각각 96×32 px, 간격 8 px |
| 색상 | 평면 charcoal surface와 절제한 mint. 본문·보조 text/input 대비 계산은 12.04:1·7.18:1, 활성 primary text 대비 11.67:1. disabled는 별도 상태 |
| 이미지·asset | 사진·로고·장식 asset 없음. native form과 text 중심 UI이며 화면 전체를 이미지로 덮어씌우지 않음. 최종 PNG는 실제 HTML 렌더 캡처 |
| 문구·정보 | 지원 코덱은 한 줄 정보. 가용성 별도 영역 제거. 비활성 이유는 선택 목록에 표시. 현재 상태·편집 값·저장 결과를 구분. UI 시안·예시 데이터 표시 유지 |

## 크기와 반응형 검증

| 항목 | 결과 |
| --- | --- |
| 기본 viewport | 1,280×880 CSS px, devicePixelRatio=1, PNG 1,280×880 |
| 최소 viewport | 760×640 CSS px, devicePixelRatio=1, PNG 760×640 |
| 최소 창 form | 1열, 상태는 3열×2행, 본문 세로 스크롤 |
| footer | 760 px 창의 x=200~760, y=553~640에 독립 배치. 버튼의 bottom=600, 가로 잘림 없음 |
| 확대·축소 | 이미지 자체를 확대해서 글꼴 크기를 판정하지 않고 CSS px와 실제 rendering을 함께 확인 |

## 상호작용 검증

- 품질 변경: footer가 미저장 상태로 전환되고 저장·적용 활성화, 현재 품질은 유지.
- 저장만: 저장 완료 표시, 현재 품질 유지, 적용만 활성화.
- 설정 적용: 예시 active 상태 변경, 변경이 없어진 버튼 비활성화.
- 기기 전환: 미저장 시 계속 편집/변경 버리기 dialog. 계속 편집은 현재 기기·draft 유지, 버리기는 다음 기기로 이동.
- 미연결 스피커: 재생 대기, 전송률 해당 없음, 적용 불가.
- 검색·전체 기기 filter: 빈 결과 안내, 비오디오 기기 표시와 설정 불가.
- 탭 방향키: 선택 탭과 focus가 함께 이동하며 해당 panel 표시.
- browser console: 검증 중 error/warn 0건.

## 한계와 후속 검증

- 값·장치·저장·적용은 브라우저 메모리에서만 동작하는 예시다. 새로 고침하면 초기화된다.
- native Narrator·OS DPI/text scaling·고대비·실제 드라이버·서비스 연동은 미실행이다.
- 이전 외부 설계 리뷰의 D1~D7은 별도 계약 보완이며 이 UI 시안으로 해결했다고 표시하지 않는다.
