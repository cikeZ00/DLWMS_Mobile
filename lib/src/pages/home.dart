// lib/src/pages/home.dart
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:dlwms_mobile/src/rust/api/simple.dart';
import 'package:dlwms_mobile/src/widgets/drawer.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final TextEditingController _urlController = TextEditingController();
  String _fetchedData = '';
  String _cookies = '';

  @override
  void initState() {
    super.initState();
    _loadCookies();
  }

  Future<void> _loadCookies() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      _cookies = prefs.getString('cookies') ?? '';
    });
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
      drawer: const GlobalDrawer(),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: <Widget>[
            TextField(
              controller: _urlController,
              decoration: const InputDecoration(labelText: 'URL to fetch data'),
            ),
            const SizedBox(height: 20),
            ElevatedButton(
              onPressed: () => _fetchData(_cookies),
              child: const Text('Fetch Data'),
            ),
            const SizedBox(height: 20),
            Text(_fetchedData),
          ],
        ),
      ),
    );
  }
}