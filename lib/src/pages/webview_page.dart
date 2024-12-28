import 'package:flutter/material.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:shared_preferences/shared_preferences.dart';

class WebViewPage extends StatefulWidget {
  const WebViewPage({Key? key}) : super(key: key);

  @override
  _WebViewPageState createState() => _WebViewPageState();
}

class _WebViewPageState extends State<WebViewPage> {
  late WebViewController _controller;
  final List<String> blockedDomains = ['www.youtube.com', 'www.facebook.com'];

  @override
  void initState() {
    super.initState();
    _initializeWebViewController();
    _loadCookies();
  }

  void _initializeWebViewController() {
    _controller = WebViewController()
      ..setJavaScriptMode(JavaScriptMode.unrestricted)
      ..setNavigationDelegate(
        NavigationDelegate(
          onProgress: (int progress) {
            debugPrint('Loading progress: $progress%');
          },
          onPageStarted: (String url) {
            debugPrint('Page started loading: $url');
          },
          onPageFinished: (String url) {
            debugPrint('Page finished loading: $url');
          },
          onHttpError: (HttpResponseError error) {
            debugPrint('HTTP error: ${error.response}');
          },
          onWebResourceError: (WebResourceError error) {
            debugPrint('Resource error: ${error.description}');
          },
          onNavigationRequest: (NavigationRequest request) {
            for (var domain in blockedDomains) {
              if (request.url.contains(domain)) {
                debugPrint('Navigation blocked to ${request.url}');
                return NavigationDecision.prevent;
              }
            }
            return NavigationDecision.navigate;
          },
        ),
      )
      ..loadRequest(
        Uri.parse('https://www.fit.ba/student'),
        headers: {
          'User-Agent': 'DLWMS/1.0.0',
        },
      );
  }

  Future<void> _loadCookies() async {
    final prefs = await SharedPreferences.getInstance();
    final cookies = prefs.getString('cookies') ?? '';
    if (cookies.isNotEmpty && cookies.contains('=')) {
      final parts = cookies.split('=');
      final cookieManager = WebViewCookieManager();
      await cookieManager.setCookie(
        WebViewCookie(
          name: parts[0],
          value: parts[1],
          domain: 'www.fit.ba',
        ),
      );
    } else {
      debugPrint('Invalid cookie format');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Student Portal'),
      ),
      body: WebViewWidget(controller: _controller),
    );
  }
}
