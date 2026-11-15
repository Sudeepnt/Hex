// Stand-in for src/apple_intelligence.swift, compiled only when that file
// can't be: without Xcode there is no FoundationModels macro plugin for
// `@Generable`. It exports the same functions and reports Apple Intelligence
// as unavailable, which Utter already handles (meeting notes fall back).
// Dictation never uses it. Build with Xcode to get the real bridge.

import Foundation

private let unavailable = "Apple Intelligence isn't available in this build"

private func refuse(_ errorOutput: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?)
    -> UnsafeMutablePointer<CChar>?
{
    errorOutput?.pointee = strdup(unavailable)
    return nil
}

/// 0 available, 1 device not eligible, 2 Apple Intelligence off,
/// 3 model not ready, 4 unsupported macOS, 5 unknown reason.
@_cdecl("utter_apple_intelligence_availability")
public func utterAppleIntelligenceAvailability() -> Int32 { 5 }

@_cdecl("utter_apple_intelligence_context_size")
public func utterAppleIntelligenceContextSize() -> Int { 0 }

@_cdecl("utter_apple_intelligence_notes")
public func utterAppleIntelligenceNotes(
    _ instructions: UnsafePointer<CChar>?,
    _ prompt: UnsafePointer<CChar>?,
    _ titleOnly: Bool,
    _ timeoutMs: UInt64,
    _ errorOutput: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? { refuse(errorOutput) }

@_cdecl("utter_apple_intelligence_chapter")
public func utterAppleIntelligenceChapter(
    _ instructions: UnsafePointer<CChar>?,
    _ prompt: UnsafePointer<CChar>?,
    _ timeoutMs: UInt64,
    _ errorOutput: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? { refuse(errorOutput) }

@_cdecl("utter_apple_intelligence_chapter_lines")
public func utterAppleIntelligenceChapterLines(
    _ instructions: UnsafePointer<CChar>?,
    _ prompt: UnsafePointer<CChar>?,
    _ timeoutMs: UInt64,
    _ errorOutput: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? { refuse(errorOutput) }

@_cdecl("utter_apple_intelligence_text")
public func utterAppleIntelligenceText(
    _ instructions: UnsafePointer<CChar>?,
    _ prompt: UnsafePointer<CChar>?,
    _ timeoutMs: UInt64,
    _ errorOutput: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? { refuse(errorOutput) }

@_cdecl("utter_apple_intelligence_free_string")
public func utterAppleIntelligenceFreeString(_ string: UnsafeMutablePointer<CChar>?) {
    free(string)
}
