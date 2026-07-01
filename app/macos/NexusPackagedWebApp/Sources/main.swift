import AppKit
import Foundation
import WebKit

struct PackageManifest: Decodable {
    let appName: String
    let slug: String
}

final class AppDelegate: NSObject, NSApplicationDelegate, WKNavigationDelegate {
    private var window: NSWindow?
    private var serverProcess: Process?

    func applicationDidFinishLaunching(_ notification: Notification) {
        let resources = Bundle.main.resourceURL ?? URL(fileURLWithPath: ".")
        let appDirectory = resources.appendingPathComponent("app", isDirectory: true)
        let manifest = readManifest(from: appDirectory)
        let title = manifest?.appName ?? "Nexus Packaged App"

        let webView = WKWebView(frame: NSRect(x: 0, y: 0, width: 1280, height: 800))
        webView.navigationDelegate = self
        webView.allowsBackForwardNavigationGestures = true

        let window = NSWindow(
            contentRect: webView.frame,
            styleMask: [.titled, .closable, .miniaturizable, .resizable],
            backing: .buffered,
            defer: false
        )
        window.title = title
        window.center()
        window.contentView = webView
        window.makeKeyAndOrderFront(nil)
        window.orderFrontRegardless()
        self.window = window
        NSApp.activate(ignoringOtherApps: true)

        if let url = startStaticServer(root: appDirectory) {
            webView.load(URLRequest(url: url))
        } else {
            let index = appDirectory.appendingPathComponent("index.html")
            webView.loadFileURL(index, allowingReadAccessTo: appDirectory)
        }
    }

    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        true
    }

    func applicationWillTerminate(_ notification: Notification) {
        serverProcess?.terminate()
    }

    private func readManifest(from appDirectory: URL) -> PackageManifest? {
        let manifestURL = appDirectory.appendingPathComponent("nexus-package-manifest.json")
        guard let data = try? Data(contentsOf: manifestURL) else {
            return nil
        }
        return try? JSONDecoder().decode(PackageManifest.self, from: data)
    }

    private func startStaticServer(root: URL) -> URL? {
        let executable = Bundle.main.bundleURL
            .appendingPathComponent("Contents")
            .appendingPathComponent("MacOS")
            .appendingPathComponent("nexus-static-server")
        guard FileManager.default.isExecutableFile(atPath: executable.path) else {
            return nil
        }

        let process = Process()
        process.executableURL = executable
        process.arguments = ["--root", root.path, "--host", "127.0.0.1", "--port", "0"]

        let pipe = Pipe()
        process.standardOutput = pipe
        process.standardError = FileHandle.standardError

        do {
            try process.run()
        } catch {
            return nil
        }

        serverProcess = process
        let data = pipe.fileHandleForReading.availableData
        guard let line = String(data: data, encoding: .utf8)?
            .split(separator: "\n")
            .first(where: { $0.hasPrefix("NEXUS_STATIC_SERVER_URL=") }) else {
            return nil
        }
        let rawURL = line.replacingOccurrences(of: "NEXUS_STATIC_SERVER_URL=", with: "")
        return URL(string: rawURL)
    }
}

let app = NSApplication.shared
let delegate = AppDelegate()
app.delegate = delegate
app.setActivationPolicy(.regular)
app.run()
