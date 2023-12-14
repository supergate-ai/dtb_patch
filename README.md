## SG-ONX-DEV-001, SG-JNX-DEV-001 캐리어보드 디바이스트리 자동 설정 프로그램

NVIDIA Jetson Xavier NX 모듈을 사용중인 경우 SD카드 슬롯 활성화를 위해 디바이스 트리 수정이 필요합니다.
NVIDIA Jetson Orin NX / Nano 모듈을 사용중인 경우 Jetson Xavier NX 모듈과 달리 CSI0, CSI1 핀의 레인 스왑으로 인해 디바이스 트리 수정이 필요합니다.

이 프로그램은 해당 과정을 자동으로 수행해줍니다.

자세한 내용은 [캐리어보드 사용자 매뉴얼](https://supergate.atlassian.net/wiki/spaces/edge/pages/579534895/CSI+Camera)을 참조바랍니다.

### Quick Setup

```
$ sudo apt update
$ sudo apt install git device-tree-compiler
$ git clone https://github.com/supergate-ai/dtb_patch
$ cd dtb_patch
$ sudo ./dtb_patch
```

### (Optional) Build from source

```
$ sudo apt update
$ sudo apt install git device-tree-compiler curl
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs | sh
> Proceed with installation (default)
$ git clone https://github.com/supergate-ai/dtb_patch
$ cd dtb_patch
$ cargo build
$ sudo target/debug/dtb_patch
```
