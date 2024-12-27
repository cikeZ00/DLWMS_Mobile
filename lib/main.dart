import 'package:dlwms_mobile/src/rust/api/simple.dart';
import 'package:dlwms_mobile/src/rust/frb_generated.dart';
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:dlwms_mobile/src/pages/login.dart';
import 'package:dlwms_mobile/src/pages/home.dart';
import 'package:dlwms_mobile/src/lib/theme_provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'DLWMS',
      theme: ThemeProvider.theme,
      home: const InitialPage(),
    );
  }
}

class InitialPage extends StatefulWidget {
  const InitialPage({super.key});

  @override
  _InitialPageState createState() => _InitialPageState();
}

class _InitialPageState extends State<InitialPage> {
  String _cookies = '';
  bool _cookiesValid = false;

  @override
  void initState() {
    super.initState();
    _loadCookies();
  }

  Future<void> _loadCookies() async {
    final prefs = await SharedPreferences.getInstance();
    final cookies = prefs.getString('cookies') ?? '';
    if (cookies.isNotEmpty) {
      _cookiesValid = await _validateCookies(cookies);
    }
    setState(() {
      _cookies = cookies;
    });
  }

  Future<bool> _validateCookies(String cookies) async {
    try {
      final response = await validateCookiesSync(cookies: cookies);
      return response.isValid;
    } catch (e) {
      return false;
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_cookies.isEmpty || !_cookiesValid) {
      return const LoginPage();
    } else {
      return const MyHomePage();
    }
  }
}