import UIKit
import WebKit

@main
final class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        let window = UIWindow(frame: UIScreen.main.bounds)
        let webView = WKWebView(frame: window.bounds)
        webView.autoresizingMask = [.flexibleWidth, .flexibleHeight]

        if let index = Bundle.main.url(forResource: "index", withExtension: "html", subdirectory: "app"),
           let appRoot = Bundle.main.url(forResource: "app", withExtension: nil) {
            webView.loadFileURL(index, allowingReadAccessTo: appRoot)
        }

        window.rootViewController = WebViewController(webView: webView)
        window.makeKeyAndVisible()
        self.window = window
        return true
    }
}

final class WebViewController: UIViewController {
    private let webView: WKWebView

    init(webView: WKWebView) {
        self.webView = webView
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func loadView() {
        view = webView
    }
}
