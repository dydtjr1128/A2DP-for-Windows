# 장치 설정 시안 제작 기록

- 날짜: 2026-10-03 (KST)
- 수정 요구: 화면 대비 큰 글자 축소, 버튼 일관성, 폰트·UI 위계 정돈, 어색한 앱 가용성 영역 제거
- 배치 참고: 내장 ImageGen으로 기존 선택 시안을 편집한 [layout-reference.png](layout-reference.png)
- 구현 기준: [device-settings.html](device-settings.html)의 system font와 CSS 크기
- 최종 화면: HTML을 1,280×880 CSS px, device scale factor 1로 캡처한 [device-settings.png](device-settings.png)
- 생성 이미지 치수: 요청 1,280×880, 실제 1,513×1,039 px. 생성 이미지의 근사 글자 크기를 그대로 최종 규격으로 채택하지 않음
- [검증 기록](design-qa.md): 모의 편집/저장/적용, 실제 font·control 치수, 760×640 창, keyboard와 console 확인
- Windows 앱·encoder·driver·실측 데이터가 아닌 설계용 HTML 시안

## 로컬에서 보기

HTML 파일을 브라우저에서 직접 열 수 있다. 네트워크·추가 dependency 없이 동작하며 예시 상태는 새로 고침 시 초기화된다.

기기 목록과 상태는 합성 데이터다. Windows Bluetooth 설정 링크는 운영체제 설정 화면으로 연결되며, 그 외 control은 실제 Windows·Bluetooth 설정을 변경하지 않는다.

## 배치 참고용 편집 프롬프트

내장 ImageGen 사용. 기존 프로젝트 시안을 이미지로 첨부했다.

```text
Edit the supplied A2DP for Windows settings concept. The user rejects its oversized typography and inconsistent buttons. Produce ONE revised, compact, credible Windows desktop utility screen at exactly 1280 x 880 pixels, 100% scale. Keep the charcoal/mint direction and left-device/right-settings arrangement; strongly reduce the typography and controls relative to the frame. This must look like a real utility screenshot at 1x, not a zoomed-in presentation board.

Typography is the main correction: Segoe UI Variable / clean Korean system sans, body and controls 14 px regular, secondary labels 12 px, device title only 20 px semibold, current values only 14 px semibold. No display type, no bold huge device heading, no large numerals. Do not increase font sizes to fill the canvas. Flat solid surfaces, no glow, gradients, glass, inset shadows, bevels, oversized outlines, decorative icon, or oversized footer. All controls and all action buttons are exactly 32 px tall with 4 px radius and a single consistent thin border. Footer actions all the same 96 px width, same 14 px font, same 32 px height. Only the primary button has mint fill.

Composition: a restrained 36 px title bar with A2DP for Windows in 13 px. Left sidebar 224 px wide, background #1B2024, main background #20262B. Sidebar title Bluetooth devices in Korean '블루투스 기기' 14 px semibold; a 32 px search field '기기 검색', a small two-option filter '오디오 기기' / '전체 장치'. Device list is unboxed flat rows of 56 px with small text: selected '책상 헤드폰' and subtitle '연결됨', '휴대용 스피커' and subtitle '연결 안 됨'. No extra settings button inside the device rows. Bottom left quiet, consistent 32 px text actions '목록 새로 고침', 'Windows Bluetooth 설정'. Use no decorative illustrations and no icons; focus on exact component proportions.

Main padding 28 px. Heading '책상 헤드폰', subtle 12 px line 'Windows 연결됨 · 오디오 전송 중'. Below, a SINGLE quiet plain-text line '지원 코덱   SBC · AAC · LDAC' in 13 px. These codec names are information, never boxed buttons. DELETE '앱에서 사용 가능' and DELETE the 'AAC · LDAC 준비 중' block entirely. Availability reasons belong only inside the codec dropdown when needed.

Then a compact current-status strip about 88 px tall, with six equal columns, tiny labels above normal-sized values: '코덱' SBC; '채널 모드' Joint Stereo; '샘플링 주파수' 48 kHz; '입력 PCM' 16 bit PCM; '전송률' 328 kbps; '품질' 균형. Minimal separators, no cards, no giant numbers. A small note below can read '오디오 데이터 기준 · 1초 평균'.

Tabs: '오디오 설정' selected, '진단', '장치 정보' in 14 px with restrained mint underline. Settings area is neatly aligned, two columns of label/control pairs, consistent widths and 44-48 px row rhythm. Show actual controls: '코덱' SBC dropdown, '채널 모드' Joint Stereo dropdown, '샘플링 주파수' 48 kHz dropdown, '입력 PCM' 16 bit PCM dropdown. A second understated group has '품질' segmented selection '안정성 / 균형 / 음질' and '오디오 버퍼' '자동 (권장)' dropdown. Segment group has same height/radius/font as other controls. One collapsed full-width '고급 설정' row. Do not add generic explanatory paragraphs below every control.

Quiet bottom footer: small '변경 사항 없음' on left; on right three equal 32px-high buttons '변경 취소', '저장만', '설정 적용', last one soft mint #A0E8CD with #10231B text. Put unobtrusive 'UI 시안 · 예시 데이터' in 12 px at bottom. No connect/disconnect action. No other product/project names, URLs, battery/signal estimates, album art or dashboard embellishments. Preserve legibility through proper 1x rendering, not oversized text. One final refined screen only.
```
