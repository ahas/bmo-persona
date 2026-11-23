[English](README.md) | **한국어** | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

# bmo-persona

채팅 에이전트의 페르소나 레이어와 로어(lore) 라이브러리를 파일로 보관합니다. 기본 레이어는 에이전트의 시스템
프롬프트에 들어가는 페르소나이고, 톤 레이어와 방 레이어는 방마다 그 위에 덧씌워집니다. 로어 라이브러리는
에이전트에게 참고 자료로 제공되는 청크의 색인입니다. 레이어 안의 개인 값은 자리표시자이며, 배포가 자체 설정에서
채워 넣습니다. 이 저장소는 서비스를 실행하지 않습니다. 검사는 내용을 읽는 쪽이 기대하는 형식에 맞는지 확인하고,
배포는 내용을 읽는 쪽이 찾는 위치에 놓으며, 설치 프로그램은 이를 채팅 에이전트의 브레인을 위한 브레인 파일로
씁니다.

## 구조

| 폴더 | 내용 |
|---|---|
| `layers/` | 기본 레이어, 톤 레이어, 그리고 `rooms/` 아래의 방 레이어 |
| `lore/` | 로어 색인, `chunks/` 아래의 청크, 청크를 잘라낸 원본 문서 |
| `src/` | 설치 프로그램: `docs/format.md`에 따라 내용을 읽어 브레인 파일로 씀 |
| `tests/` | 내용을 형식에, 저장소를 규칙에 맞게 지키는 검사와 설치 프로그램의 테스트 |
| `docs/` | 형식, 경계, 목록에 대한 정식 문서 |

루트에는 설치 프로그램과 검사의 Cargo 매니페스트, 버전, 한국어 원장(ledger)을 동반한 `testbed.toml`,
`rustfmt.toml`, MIT `LICENSE`, `CLAUDE.md`가 있습니다.

## 문서

| 문서 | 내용 |
|---|---|
| `docs/format.md` | 레이어와 그 톤 줄의 형태, 로어 색인과 청크, 자리표시자 |
| `docs/boundary.md` | 저장소가 제공하는 것, 배포가 이를 놓는 방식, 설치 프로그램이 쓰는 브레인 파일, 어느 유닛이 무엇을 받는지, 소스 트리를 읽는 각 쪽이 어디로 가는지 |
| `docs/inventory.md` | 모든 내용 파일의 출처와 경로, 제외한 것, 개인 값을 대체한 모든 자리표시자 |
| `docs/PERSONAL-DATA.md` | 저장소에 절대 들어가지 않는 값, 허용하는 두 가지, 이를 지키는 검사 |

## 버전

- **저장소 전체에 하나의 버전**을 semver로 씁니다. `VERSION`이 이를 담고, 패키지의 버전이 이와 같습니다.
- **변경이 버전을 어떻게 올리는지**는 `docs/boundary.md`의 「What this repository is」에 있습니다.

## 명령

```sh
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --bin bmo-persona-install -- --out <file>
```

마지막 명령은 브레인 파일을 씁니다(`docs/boundary.md`, 「The brain file」). 이를 브레인에 주입하는 것은 운영자의
몫입니다.
