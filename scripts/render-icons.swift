// Draws Utter's mark and every icon file the app ships, from code.
//
//   swift scripts/render-icons.swift
//
// The mark is a rounded "u" holding a level bar: a lowercase u and a
// microphone at once. Change BRAND_TOP / BRAND_BOTTOM to recolour the app
// icon; the menu bar and tray icons are single-colour and follow the system.

import AppKit
import CoreGraphics
import Foundation
import ImageIO
import UniformTypeIdentifiers

let BRAND_TOP = CGColor(srgbRed: 0.66, green: 0.40, blue: 0.97, alpha: 1)     // #a866f7
let BRAND_BOTTOM = CGColor(srgbRed: 0.40, green: 0.13, blue: 0.74, alpha: 1)  // #6621bd

let root = URL(fileURLWithPath: CommandLine.arguments.count > 1
    ? CommandLine.arguments[1]
    : FileManager.default.currentDirectoryPath)

/// Draws the mark in a 100×100 box (y down) scaled into `rect`.
func drawMark(_ ctx: CGContext, in rect: CGRect, color: CGColor, dot: Bool = false) {
    ctx.saveGState()
    ctx.translateBy(x: rect.minX, y: rect.minY)
    ctx.scaleBy(x: rect.width / 100, y: rect.height / 100)
    ctx.setStrokeColor(color)
    ctx.setLineCap(.round)
    ctx.setLineJoin(.round)

    // The u: two stems joined by a half circle.
    let u = CGMutablePath()
    u.move(to: CGPoint(x: 24, y: 18))
    u.addLine(to: CGPoint(x: 24, y: 54))
    u.addArc(center: CGPoint(x: 50, y: 54), radius: 26,
             startAngle: .pi, endAngle: 0, clockwise: true)
    u.addLine(to: CGPoint(x: 76, y: 18))
    ctx.setLineWidth(13)
    ctx.addPath(u)
    ctx.strokePath()

    // The level bar inside the bowl.
    ctx.setLineWidth(13)
    ctx.move(to: CGPoint(x: 50, y: 24))
    ctx.addLine(to: CGPoint(x: 50, y: 56))
    ctx.strokePath()

    if dot {
        // A recording dot, cut clear of the mark.
        ctx.setBlendMode(.clear)
        ctx.fillEllipse(in: CGRect(x: 62, y: 62, width: 40, height: 40))
        ctx.setBlendMode(.normal)
        ctx.setFillColor(color)
        ctx.fillEllipse(in: CGRect(x: 69, y: 69, width: 26, height: 26))
    }
    ctx.restoreGState()
}

/// A bitmap context with y pointing down, like the mark's coordinates.
func canvas(_ w: Int, _ h: Int) -> CGContext {
    let ctx = CGContext(data: nil, width: w, height: h, bitsPerComponent: 8, bytesPerRow: w * 4,
                        space: CGColorSpace(name: CGColorSpace.sRGB)!,
                        bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)!
    ctx.translateBy(x: 0, y: CGFloat(h))
    ctx.scaleBy(x: 1, y: -1)
    ctx.interpolationQuality = .high
    ctx.setShouldAntialias(true)
    return ctx
}

func pngData(_ ctx: CGContext) -> Data {
    let data = NSMutableData()
    let dest = CGImageDestinationCreateWithData(data, UTType.png.identifier as CFString, 1, nil)!
    CGImageDestinationAddImage(dest, ctx.makeImage()!, nil)
    CGImageDestinationFinalize(dest)
    return data as Data
}

func write(_ data: Data, _ path: String) {
    let url = root.appendingPathComponent(path)
    try! FileManager.default.createDirectory(at: url.deletingLastPathComponent(),
                                             withIntermediateDirectories: true)
    try! data.write(to: url)
    print("wrote \(path)")
}

/// The full-colour app icon: the mark in white on a brand gradient squircle.
func appIcon(_ size: Int) -> CGContext {
    let ctx = canvas(size, size)
    let s = CGFloat(size)
    let inset = s * 0.06
    let tile = CGRect(x: inset, y: inset, width: s - 2 * inset, height: s - 2 * inset)
    let path = CGPath(roundedRect: tile, cornerWidth: tile.width * 0.225,
                      cornerHeight: tile.width * 0.225, transform: nil)
    ctx.addPath(path)
    ctx.clip()
    let gradient = CGGradient(colorsSpace: CGColorSpace(name: CGColorSpace.sRGB)!,
                              colors: [BRAND_TOP, BRAND_BOTTOM] as CFArray, locations: [0, 1])!
    ctx.drawLinearGradient(gradient, start: CGPoint(x: 0, y: tile.minY),
                           end: CGPoint(x: 0, y: tile.maxY), options: [])
    let m = tile.width * 0.56
    drawMark(ctx, in: CGRect(x: tile.midX - m / 2, y: tile.midY - m / 2, width: m, height: m),
             color: CGColor(gray: 1, alpha: 1))
    return ctx
}

func markOnly(_ size: Int, color: CGColor, dot: Bool = false, pad: CGFloat = 0.08) -> CGContext {
    let ctx = canvas(size, size)
    let s = CGFloat(size)
    drawMark(ctx, in: CGRect(x: s * pad, y: s * pad, width: s * (1 - 2 * pad), height: s * (1 - 2 * pad)),
             color: color, dot: dot)
    return ctx
}

/// Straight (not premultiplied) RGBA bytes, as the Windows tray reads them.
func straightRGBA(_ ctx: CGContext) -> Data {
    let bytes = ctx.data!.assumingMemoryBound(to: UInt8.self)
    var out = Data(count: ctx.width * ctx.height * 4)
    for i in 0..<(ctx.width * ctx.height) {
        let a = bytes[i * 4 + 3]
        out[i * 4 + 3] = a
        for c in 0..<3 {
            out[i * 4 + c] = a == 0 ? 0 : UInt8(min(255, Int(bytes[i * 4 + c]) * 255 / Int(a)))
        }
    }
    return out
}

// macOS: the app icon layer (white mark, Icon Composer adds the fill) and the
// menu bar template images.
write(pngData(markOnly(1600, color: CGColor(gray: 1, alpha: 1), pad: 0)), "app/AppIcon.icon/Assets/Image.png")
write(pngData(markOnly(36, color: CGColor(gray: 0, alpha: 1), pad: 0.1)), "assets/icons/menubar-idle@2x.png")
write(pngData(markOnly(36, color: CGColor(gray: 0, alpha: 1), dot: true, pad: 0.1)), "assets/icons/menubar-recording@2x.png")

// Windows: window icons, the tray's raw pixels, and the .ico's frames.
for size in [32, 64, 128] {
    write(pngData(appIcon(size)), "resources/windows/icon-\(size).png")
}
write(straightRGBA(markOnly(32, color: CGColor(gray: 1, alpha: 1), pad: 0.06)), "resources/windows/tray-32.rgba")
for size in [16, 24, 32, 48, 64, 128, 256] {
    write(pngData(appIcon(size)), "target/icon-frames/\(size).png")
}

// Linux: the hicolor app icons.
for size in [16, 24, 32, 48, 64, 128, 256, 512] {
    write(pngData(appIcon(size)), "packaging/linux/icons/hicolor/\(size)x\(size)/apps/app.utter.png")
}
