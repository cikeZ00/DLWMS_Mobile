import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:dlwms_mobile/src/rust/api/simple.dart';
import 'home.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({super.key});

  @override
  _LoginPageState createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  String _loginMessage = '';
  String _selectedInstitute = '1';

  final List<Map<String, String>> _institutes = [
    {'value': '1', 'name': 'Fakultet informacijskih tehnologija'},
    {'value': '2', 'name': 'FIT (Postdiplomski studij)'},
    {'value': '3', 'name': 'FHN'},
    {'value': '4', 'name': 'DEMO'},
    {'value': '5', 'name': 'Poslovna informatika'},
  ];

  Future<void> _storeCookies(String cookies) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('cookies', cookies);
  }

  Future<void> _login() async {
    final username = _usernameController.text;
    final password = _passwordController.text;
    final institute = _selectedInstitute;

    try {
      final response = await loginSync(
        username: username,
        password: password,
        institute: institute,
      );
      setState(() {
        _loginMessage = response.message;
        if (response.success && response.cookies != null) {
          _storeCookies(response.cookies!);
          Navigator.pushReplacement(
            context,
            MaterialPageRoute(builder: (context) => const MyHomePage()),
          );
        }
      });
    } catch (e) {
      setState(() {
        _loginMessage = 'Login failed: $e';
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Login'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: <Widget>[
            TextField(
              controller: _usernameController,
              decoration: const InputDecoration(labelText: 'Username'),
              autofillHints: const [AutofillHints.username],
            ),
            TextField(
              controller: _passwordController,
              decoration: const InputDecoration(labelText: 'Password'),
              obscureText: true,
              autofillHints: const [AutofillHints.password],
            ),
            DropdownButton<String>(
              value: _selectedInstitute,
              onChanged: (String? newValue) {
                setState(() {
                  _selectedInstitute = newValue!;
                });
              },
              items: _institutes.map<DropdownMenuItem<String>>((Map<String, String> institute) {
                return DropdownMenuItem<String>(
                  value: institute['value'],
                  child: Text(institute['name']!),
                );
              }).toList(),
            ),
            const SizedBox(height: 20),
            ElevatedButton(
              onPressed: _login,
              child: const Text('Login'),
            ),
            const SizedBox(height: 20),
            Text(_loginMessage),
          ],
        ),
      ),
    );
  }
}