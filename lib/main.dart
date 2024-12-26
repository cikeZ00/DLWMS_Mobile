import 'package:flutter/material.dart';
import 'package:dlwms_mobile/src/rust/api/simple.dart';
import 'package:dlwms_mobile/src/rust/frb_generated.dart';

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
      title: 'DLWMS But Good',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final TextEditingController _instituteController = TextEditingController();
  final TextEditingController _urlController = TextEditingController();
  String _loginMessage = '';
  String _fetchedData = '';
  String _cookies = '';

  Future<void> _login() async {
    final username = _usernameController.text;
    final password = _passwordController.text;
    final institute = _instituteController.text;

    try {
      final response = await loginSync(
        username: username,
        password: password,
        institute: institute,
      );
      setState(() {
        _loginMessage = response.message;
        if (response.success && response.cookies != null) {
          _cookies = response.cookies!;
          print('Cookies: $_cookies');
          _fetchData(response.cookies!);
        }
      });
    } catch (e) {
      setState(() {
        _loginMessage = 'Login failed: $e';
      });
    }
  }

  Future<void> _fetchData(String cookies) async {
    final url = _urlController.text;

    try {
      final response = await requestPageSync(
        url: url,
        cookies: cookies,
      );
      setState(() {
        _fetchedData = response.page;
      });
    } catch (e) {
      setState(() {
        _fetchedData = 'Fetch failed: $e';
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('DLWMS But Good'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: <Widget>[
            TextField(
              controller: _usernameController,
              decoration: const InputDecoration(labelText: 'Username'),
            ),
            TextField(
              controller: _passwordController,
              decoration: const InputDecoration(labelText: 'Password'),
              obscureText: true,
            ),
            TextField(
              controller: _instituteController,
              decoration: const InputDecoration(labelText: 'Institute'),
            ),
            const SizedBox(height: 20),
            ElevatedButton(
              onPressed: _login,
              child: const Text('Login'),
            ),
            const SizedBox(height: 20),
            Text(_loginMessage),
            const SizedBox(height: 20),
            TextField(
              controller: _urlController,
              decoration: const InputDecoration(labelText: 'URL to fetch data'),
            ),
            const SizedBox(height: 20),
            Text(_fetchedData),
          ],
        ),
      ),
    );
  }
}