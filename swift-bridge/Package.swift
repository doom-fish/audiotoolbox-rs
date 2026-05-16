// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "AudioToolboxBridge",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "AudioToolboxBridge",
            type: .static,
            targets: ["AudioToolboxBridge"]
        )
    ],
    targets: [
        .target(
            name: "AudioToolboxBridge",
            path: "Sources/AudioToolboxBridge",
            publicHeadersPath: "include"
        )
    ]
)
