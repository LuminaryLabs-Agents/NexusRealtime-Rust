import AppKit
import Foundation

@_silgen_name("nexus_host_demo_status")
func nexus_host_demo_status() -> UnsafeMutablePointer<CChar>?

@_silgen_name("nexus_host_string_free")
func nexus_host_string_free(_ value: UnsafeMutablePointer<CChar>?)

final class AppDelegate: NSObject, NSApplicationDelegate {
    private var window: NSWindow?

    func applicationDidFinishLaunching(_ notification: Notification) {
        let status = readRustStatus()
        let view = DemoView(frame: NSRect(x: 0, y: 0, width: 720, height: 420), status: status)
        let window = NSWindow(
            contentRect: view.frame,
            styleMask: [.titled, .closable, .miniaturizable, .resizable],
            backing: .buffered,
            defer: false
        )
        window.title = "NexusEngine Rust Demo"
        window.center()
        window.contentView = view
        window.makeKeyAndOrderFront(nil)
        window.orderFrontRegardless()
        self.window = window
        NSApp.activate(ignoringOtherApps: true)
    }

    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        true
    }
}

final class DemoView: NSView {
    private let status: String

    init(frame frameRect: NSRect, status: String) {
        self.status = status
        super.init(frame: frameRect)
        wantsLayer = true
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func draw(_ dirtyRect: NSRect) {
        NSColor(calibratedRed: 0.08, green: 0.10, blue: 0.12, alpha: 1.0).setFill()
        dirtyRect.fill()

        drawTitle("NexusEngine Rust Demo", at: CGPoint(x: 32, y: bounds.height - 70), size: 30, color: .white)
        drawTitle("Rust backend loaded through libnexus_host_ffi.dylib", at: CGPoint(x: 34, y: bounds.height - 108), size: 15, color: NSColor(calibratedRed: 0.74, green: 0.82, blue: 0.90, alpha: 1.0))

        let card = NSRect(x: 32, y: 42, width: bounds.width - 64, height: bounds.height - 178)
        NSColor(calibratedRed: 0.14, green: 0.18, blue: 0.22, alpha: 1.0).setFill()
        NSBezierPath(roundedRect: card, xRadius: 8, yRadius: 8).fill()

        let paragraph = NSMutableParagraphStyle()
        paragraph.lineSpacing = 5
        let attrs: [NSAttributedString.Key: Any] = [
            .font: NSFont.monospacedSystemFont(ofSize: 14, weight: .regular),
            .foregroundColor: NSColor(calibratedRed: 0.88, green: 0.93, blue: 0.96, alpha: 1.0),
            .paragraphStyle: paragraph
        ]
        status.draw(in: card.insetBy(dx: 22, dy: 22), withAttributes: attrs)
    }

    private func drawTitle(_ text: String, at point: CGPoint, size: CGFloat, color: NSColor) {
        let attrs: [NSAttributedString.Key: Any] = [
            .font: NSFont.systemFont(ofSize: size, weight: .semibold),
            .foregroundColor: color
        ]
        text.draw(at: point, withAttributes: attrs)
    }
}

func readRustStatus() -> String {
    guard let pointer = nexus_host_demo_status() else {
        return "Rust backend did not return a status pointer."
    }
    defer { nexus_host_string_free(pointer) }
    return String(cString: pointer)
}

let app = NSApplication.shared
let delegate = AppDelegate()
app.delegate = delegate
app.setActivationPolicy(.regular)
app.run()
