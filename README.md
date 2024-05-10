# sandbox-research

This repository contains my study on Windows internals and my research on sandbox concepts with proof-of-concepts dedicated to Windows. Links with https://github.com/aCursedComrade/sandbox-driver

## Resources

- [Hardware developer documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/)
- [WDK bindings for Rust](https://github.com/microsoft/windows-drivers-rs/)

## Build Dependencies

- MS Build Tools and Windows Driver Kit are required to compile the project including the driver. [Read the documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk) to setup the environment.
- [Protobuf compiler - `protoc`](https://github.com/protocolbuffers/protobuf?tab=readme-ov-file#protobuf-compiler-installation) used to compile Protobuf source files to be consumed by gRPC servcies.

## Runtime Dependencies

- [Visual C++ Redistributles 2015-2022](https://learn.microsoft.com/en-US/cpp/windows/latest-supported-vc-redist?view=msvc-170)
